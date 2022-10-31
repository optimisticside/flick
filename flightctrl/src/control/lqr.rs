use core::fmt;

use nalgebra::{OMatrix, OVector, RealField};
use nalgebra::allocator::Allocator;
use nalgebra::base::DefaultAllocator;
use nalgebra::base::dimension::{Dim, DimMin, DimName};

/// Linear Quadratic Regulator (LQR) feedback controller.
///
/// An LQR is able to solve the optimal control problem in a system that can be described by a set
/// of linear differential equations, and the cost can be described by a quadratic function.
///
/// Riccatti equation solved via
/// https://www.tandfonline.com/doi/abs/10.1080/00207170410001714988
/// https://scicomp.stackexchange.com/questions/30757/discrete-time-algebraic-riccati-equation-dare-solver-in-c
#[derive(Clone, Debug)]
pub struct LqrController<T, S, C>
where
    T: RealField,
    // State dimensions.
    S: Dim + DimName + DimMin<S>,
    // Control dimensions.
    C: Dim + DimName + DimMin<C>,
    DefaultAllocator:
        Allocator<T, S, S> + Allocator<T, C, C> + Allocator<T, S, C> + Allocator<T, C, S>,
{
    /// State cost.
    pub q: Option<OMatrix<T, S, S>>,
    /// Control cost.
    pub r: Option<OMatrix<T, C, C>>,
    /// Optimal gain.
    pub k: Option<OMatrix<T, C, S>>,

    /// The controller also has an i-controller for removing steady state error in y dimension
    /// i-controller coefficient.
    ki: T,
    /// Accumulating integral error for the i-controller.
    integral_error: T,
    /// Random number generator for creating random matrices.
    rand: fn() -> T,
}

impl<T, S, C> LqrController<T, S, C>
where
    T: RealField + Copy,
    S: Dim + DimName + DimMin<S>,
    C: Dim + DimName + DimMin<C>,
    DefaultAllocator:
        Allocator<T, S, S> + Allocator<T, C, C> + Allocator<T, S, C> + Allocator<T, C, S>,
{
    /// Instantiates a new controller in default mode (without the i-controller).
    pub fn new(rand: fn() -> T) -> Self {
        Self {
            q: None,
            r: None,
            k: None,
            ki: T::zero(),
            integral_error: T::zero(),
            rand,
        }
    }

    /// Computes and returns the optimal gain matrix K for the LQR controller.
    ///
    /// # Arguments
    /// * `a` - State matrix of shape SxS.
    /// * `b` - Control matrix of shape SxC.
    /// * `q` - State cost matrix of shape SxS.
    /// * `r` - Control cost amtrix of shape CxC.
    /// * `epsilon` - Small value to avoid division by 0. 1e-6 works nicely.
    ///
    /// # Returns
    /// The optimal feedback gain matrix `k` of shape CxS.
    pub fn compute_gain(
        &mut self,
        a: &OMatrix<T, S, S>,
        b: &OMatrix<T, S, C>,
        q: &OMatrix<T, S, S>,
        r: &OMatrix<T, C, C>,
        epsilon: T,
    ) -> Result<OMatrix<T, C, S>, &'static str> {
        let mut ak = a.clone();
        let mut gk = b.clone()
            * r.clone().try_inverse().expect("Couldn't compute inverse")
            * b.clone().transpose();

        let mut hk1 = q.clone();
        let mut hk = OMatrix::<T, S, S>::from_fn(|_, _| (self.rand)());

        while {
            let error = (hk1.clone() - hk).norm() / hk1.norm();
            hk = hk1.clone();
            error >= epsilon
        } {
            let temp = (OMatrix::<T, S, S>::identity() + &gk * &hk).try_inverse().expect("Couldn't compute inverse");
            let ak1 = &ak * &temp * &ak;
            let gk1 = &gk + &ak * &temp * &gk * &ak.transpose();
            hk1 = &hk + &ak.transpose() * &hk * &temp * &ak;
            ak = ak1;
            gk = gk1;
        }

        // Calculate final gain matrix.
        self.k = Some(r.clone().try_inverse().expect("Couldn't compute inverse") * b.transpose() * hk);
        return Ok(self.k.clone().unwrap());
    }

    /// Returns the optimal feedback control based on the desired and current state vectors.
    /// This should  be called only after compute_gain() has already been called.
    ///
    /// # Arguments
    /// * `current_state` - vector of length S
    /// * `desired_state` - vector of length S which we want to get to
    ///
    /// # Returns
    /// The feedback control gains. These are insufficient to control anything and have to be
    /// combined with the feedforward controls. Check examples
    pub fn compute_optimal_controls(
        &mut self,
        current_state: &OVector<T, S>,
        desired_state: &OVector<T, S>,
    ) -> Result<OVector<T, C>, &'static str>
    where
        T: RealField,
        DefaultAllocator: Allocator<T, S> + Allocator<T, C> + Allocator<T, C, S>,
    {
        let error = desired_state - current_state;
        let y_error = error[1];
        if y_error.abs() < T::from_f64(1e-2).unwrap() {
            self.integral_error = T::zero();
        } else {
            self.integral_error += y_error * self.ki.clone();
        }

        let mut controls = &self.k.clone().unwrap() * error;
        controls[0] -= self.integral_error.clone();

        Ok(controls)
    }
}

impl<T, S, C> fmt::Display for LqrController<T, S, C>
where
    T: RealField + Copy,
    S: Dim + DimName + DimMin<S>,
    C: Dim + DimName + DimMin<C>,
    DefaultAllocator:
        Allocator<T, S, S> + Allocator<T, C, C> + Allocator<T, S, C> + Allocator<T, C, S>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LqrController {{ q = {:?}, r = {:?}, ki: {:?} }}", self.q, self.r, self.ki)
    }
} 
