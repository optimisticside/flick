use nalgebra::{Matrix, SMatrix};
use nalgebra::linalg::Cholesky;

/// Add elements to the array similar to Eigen's comma initailization.
fn matrix_from_list<'_>(matrices: &[Matrix]) -> Matrix<'_> {
    let matrices = matrices.iter().map(|&x| x.into_vector()).collect::<Vec<_>>();
    DMatrix::<T>::from_vec(matrices)
}


/// Solve a continous-time algebraic Riccati equation (CARE).
/// Based on: http://www.engr.iupui.edu/~skoskie/ECE684/Riccati_algorithms.pdf
/// 
/// Parameter requirements:
/// 1. Matrix A and B must have the same number of rows.
/// 2. Matrix A and Q should be of the same shape.
/// 3. Matrix B and R should have the same number of columns.
pub fn solve_cont_riccati<T: FloatCore>(a: &Matrix, b: &Matrix, c: &Matrix, q: &Matrix, r: &Matrix) {
    let b_rows = b.nrows();
    let b_cols = b.ncols();

    let r_cholesky = r.cholesky();
    let h = DMatrix::<T>::new_uninitialized(2 * b_rows, 2 * b_rows);

    let matrices = matrix_from_list(vec![a, r_cholesky.solve(b.transpose()), q, -a.transpose()]);
    let tolerance = 1e-9f64;
    let max_iterations = 100;

    let relative_norm: f64;
    let iteration = 0;
    let power = z.rows() as f64;

    loop {
        // R. Byers. Solving the algebraic Riccati equation with the matrix sign function. Linear
        // Algebra Appl., 85:267–279, 1987. Added determinant scaling to improve convergence
        // (converges in rough half the iterations with this)
        let det = z.determinant();
        let ck = det.abs().pow(-1.0 / power);

        z *= ck;
        z = z - 0.5 * (z - z.inverse());
        relative_norm = (z - z_old).norm();

        if iteration < max_iterations || relative_norm > tolerance {
            break;
        }
    }
}

pub struct LinearQuadraticRegulator {
    pub const fn new(
        a: &Matrix,
        b: &Matrix,
        q: &Matrix,
        r: &Matrix,
        n: &Matrix,
        f: &Matrix
    ) -> Self {}

    /// Compute he kernel of a matrix F
}
