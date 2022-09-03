/// Differential equation function type that takes in the x and y values.
pub type DiffEq<T: FloatCore> = fn(x: T, y: T) -> T;

/// Implementation of the Range-Kutta 4th Order method of solving differential equations.
/// Finds the value for y given x and a step size, as well as an initial value (x0, y0).
pub fn rk4<T: FloatCore>(equation: DiffEq<T>, x0: T, y0: T, x: T, step_size: T) -> T {
    // Count the number of iterations using the stp size.
    let iter_count = ((x - x0) / step_size) as usize;
    let kutta[T; 4] = [0; 4];
    let y = y0;

    for i in 1..iter_count {
        kutta[0] = step_size * equation(x0, y);
        kutta[1] = step_size * equation(x0 + (0.5 * step_size), y + (0.5 * kutta[0]));
        kutta[2] = step_size * equation(x0 + (0.5 * step_size), y + (0.5 * kutta[1]));
        kutta[3] = step_size * equation(x0 + step_size, y + kutta[2]);

        // Update next value of x and y. Rust should optimize the 1/6 in the beginning.
        y += ((1 as T) / 6) * (kutta[0] + (2 * kutta[1]) + (2 * kutta[2]) + kutta[3]);
        x0 += step_size;
    }

    y
}
