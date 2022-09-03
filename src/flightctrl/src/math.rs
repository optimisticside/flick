/// Differential equation function type that takes in the x and y values.
pub type DiffEq<T: FloatCore> = fn(x: T, y: T) -> T;

/// Implementation of the Range-Kutta 4th Order method of solving differential equations.
/// Finds the value for y given x and a step size, as well as an initial value (x0, y0).
pub fn rk4<T: FloatCore>(equation: DiffEq<T>, mut x0: T, mut y0: T, mut x: T, step_size: T) -> T {
    // Count the number of iterations using the stp size.
    let iter_count = ((x - x0) / step_size) as usize;
    let (mut k1, mut k2, mut k3, mut k4): (T, T, T, T) = (0, 0, 0, 0);
    let mut y = y0;

    for i in 1..iter_count {
        k1 = step_size * equation(x0, y);
        k2 = step_size * equation(x0 + (0.5 * step_size), y + (0.5 * k1));
        k3 = step_size * equation(x0 + (0.5 * step_size), y + (0.5 * k2));
        k4 = step_size * equation(x0 + step_size, y + k3);

        // Update next value of x and y. Rust should optimize the 1/6 in the beginning.
        y += ((1 as T) / 6) * (k1 + (2 * k2) + (2 * k3) + k4);
        x0 += step_size;
    }

    y
}
