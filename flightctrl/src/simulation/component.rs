/// A rocket component represents anything that physically makes up the rocket. This includes
/// things like fins and the body itself. Note that everything is relative to the center of the
/// rocket body.
#[derive(Debug, Clone, Copy)]
pub struct RocketComponent {
    /// Component name (for debugging purposes only).
    name: Option<str>,
    /// Dimentions of the object's verticies relative to its center point.
    /// See [`RocketComponent::position`] for information about that.
    verticies: Vec<Vector3<f64>>,
    /// Center of the component, relative to the designated center-point of the rocket.
    position: Quaternion<f64>,
    /// The type of rocket component.
    kind: RocketComponentKind,
}

/// Types of rocket components, along with their speciric data.
pub enum RocketComponentKind {
    /// Anything that is just structural.
    Structure,
    /// Any kind of motor.
    Motor(MotorComponent),
}
