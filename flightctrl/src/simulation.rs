use nalgebra::Quaternion;
use num_traits::float::FloatCore;

/// Representation of a fluid: any liquid or gas that deforms under stress or force. Only contains
/// a density.
#[derive(Debug, Clone)]
pub struct Fluid<T: FloatCore>(T);

/// Simulates a rocket traveling in air of the given density (generally 1.225 kg/m^3) with a given
/// lift and drag coefficient (which have to be calculated manually).
#[derive(Debug, Clone, Copy)]
pub struct Rocket {
    /// Drag coefficient.
    /// See https://www.grc.nasa.gov/www/k-12/rocket/dragco.html
    drag_coeff: f64,
    /// Lift coefficient.
    /// See https://www.grc.nasa.gov/www/k-12/rocket/liftco.html
    lift_coeff: f64,
    /// Density of air. This is usually 1.225 kg/m^3, but may vary depending on location.
    air_density: f64,
    /// Mass of the rocket.
    mass: f64,
    /// Initial position.
    init_pos: Quaternion<f64>,
    /// Initial velocity.
    init_vel: Quaternion<f64>,
}
