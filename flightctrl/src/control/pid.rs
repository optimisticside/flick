use num_traits::float::FloatCore;

/// A Proportioanl-integral-derivative controller is a control loop for continously modulated
/// control. Continously calculates the error value between a desired set-point and a measured
/// process-variable.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pid<T: FloatCore> {
    /// Proportional gain.
    pub kp: T,
    /// Integral gain.
    pub ki: T,
    /// Derivative gain.
    pub kd: T,

    /// Set-point.
    pub setpoint: T,
    /// Integral term.
    pub integral: T,
    /// Previous measurement.
    pub previous: Option<T>,
}

/// Output of a PID's calculation.
#[derive(Debug)]
pub struct ControlOutput<T> {
    /// Contribution of the P term to the output.
    pub p: T,
    /// Contribution of the I term to the output.
    /// `i = sum[error(t) * ki(t)] (for all t)`
    pub i: T,
    /// Contribution of the D term to the output.
    pub d: T,
    /// Output of the PID controller.
    pub output: T,
}

// There is no need for a constructor since the object can just be constructed (no extra processing
// needs to be done).
impl<T> Pid<T>
where
    T: FloatCore
{
    /// Reset the integral term back to zero.
    fn reset_integral(&mut self) {
        self.integral = T::zero();
    }

    /// Given a new measurement, calculates the next control output.
    fn calculate(&mut self, measurement: T) -> ControlOutput<T> {
        let error = self.setpoint - measurement;
        let proportional = error * self.kp;
        let integral = self.integral + error * self.ki;
        let derivative = -match self.previous.as_ref() {
            Some(previous) => measurement - *previous,
            None => T::zero(),
        } * self.kd;

        let output = proportional + integral + derivative;
        self.previous = Some(measurement);
        self.integral = integral;

        ControlOutput {
            p: proportional,
            i: integral,
            d: derivative,
            output,
        }
    }
}
