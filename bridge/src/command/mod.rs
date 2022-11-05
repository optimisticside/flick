
use serde::{Deserialize, Serialize};

/// A command is any request that can be sent through a packet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Command {
	/// Fire at the given pyro-channel. This is usually for when manual control is necessary, and
	/// is generally not used often (Happens less often than [`Command::ChangeState`]).
	FirePyro(u16),
	/// Toggle a pyro channel.
	TogglePyro(u16),
	/// Change the state of the rocket. This is only meant for rare occations when manual control
	/// is necessary. Note that the rocket-state is not provided as an enumeration and instead is
	/// the ID.
	ChangeState(u32),
	/// Sent every now and then to ensure the bridge is still active.
	Heartbeat,
	/// Request telemetry.
	Telemetry,
}

/// Represents the response to a command.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommandResponse {
	
}