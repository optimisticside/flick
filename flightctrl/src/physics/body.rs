/// Represents a body with 6 degrees of freedom. All data is relative to the local frame.
#[derive(Debug)]
pub struct Body {
    /// Center of gravity.
    center_of_gravity: Vector3<f64>,
    /// Position.
    position: Vector3<f64>,
    /// Acceleration.
    acceleration: Vector3<f64>,
    /// Forces applied to body.
    forces: Vector3<f64>,
    /// Total mass.
    total_mass: f64,
}

impl Body {
    /// Main simulation routine. Computes the properties of the body based on the inputs persisting
    /// for the given delta time.
    fn step(&mut self, delta_time: f64) {
        for vertex in self.verticies.iter() {
            
        }
    }
}
