use crate::simulation::Motor;

/// Function that calculates the drag coefficient of a body given Mach number.
/// See [`Rocket::power_on_drag`] and [`Rocket::power_off_drag`].
type DragCoeffFunction = fn(mach: f64) -> f64;

/// Function that calculates the lift coefficient of a body given the angle of attack (alpha) in
/// radians, and the mach number (Mach).
type LiftCoeffFunction = fn(alpha: f64, mach: f64) -> f64;

/// Representation of an aerodynamic surface on the rocket.
#[derive(Clone, Copy, Debug)]
pub struct AerodynamicSurface {
    /// Function to calculate the lift coefficient.
    lift_coeff: LiftCoeffFunction,
    /// Center of pressure of the surface.
    pressure_center: Vector3<f64>,
    /// Name of the surface (for debugging purposes only).
    name: Option<str>,
}

/// Representation of a rocket body.
#[derive(Debug, Clone, Copy)]
pub struct Rocket {
    /// Largest radius of the rocket (in meters).
    radius: f64,
    /// Circular cross-section largest frontal area in squared meters.
    area: f64,
    /// Distance between the rocket's center of mass (without propellant) to the motor reference
    /// point, which for sollid and hybrid motors in the center of mass of solid propellant in
    /// meters. This is always positive.
    dist_propellant: f64,
    /// Mass without propelland in kilograms.
    mass: f64,
    /// Moment of inertia, without propellant, with respect to axis perpendicular to rocket's axis
    /// of cylindrical symmetry, in kg*m^2.
    inertia_i: f64,
    /// Moment of inertia, without propellant, with respect to the rocket's axis of cylindrical
    /// symmetry, in kg*m^2.
    inertia_z: f64,
    /// Distance of the rocket's center of mass, including propellant, to the rocket's center of of
    /// mass without propellant, in meters.
    center_of_mass: fn(ms: usize) -> f64,
    /// Center of pressure position relative to center of mass in the x and y axes, perpendicular
    /// to axis of cylindrical symmetry, in meters.
    cp_eccentricity: Vector2<f64>,
    /// Thrust vector position relative to center of mass in the x and y axes, perpendicular to
    /// axis of cylindrical symmetry, in meters.
    thrust_eccentricity: Vector2<f64>,
    /// List of aerodynamic surfaces on the rocket.
    aerodyn_surfaces: Vec<AerodynamicSurface>,
    /// Function to calculate the drag coefficient when the motor is on.
    power_on_drag: DragCoeffFunction,
    /// Function to calculate the drag coefficient when the motor is off.
    power_off_drag: DragCoeffFunction,
    /// Motor of the rocket.
    motor: Motor,
}

impl Rocket {
    /// Instantiate a new [`Rocket`].
    pub fn new(
        motor: Motor,
        mass: f64,
        inertia_i: f64,
        inertia_z: f64,
        radius: f64,
        dist_nozzle: f64,
        dist_propellant: f64,
        power_on_drag: f64,
        power_off_drag: f64,
    ) -> Result<Self, &'static str> {
        Self {
            motor,
            mass,
            inertia_i,
            inertia_z,
        }
    }
}
