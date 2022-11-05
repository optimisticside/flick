use serde::{Deserialize, Serialize};
use crate::command::{Command, CommandResponse};

bitflags::bitflags! {
	/// Flags for packets. See [`Packet::flags`].
	pub struct PacketFlags: u8 {
	}
}

/// A message is the basis for all communication operations that can be done on the bridge.
/// Messages contain the data to be sent, along with other metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
	/// Unique identifier used to reference the packet from other data, such as acknowledgement
	/// packets.
	id: u64,
	/// Data specific to the type of packet.
	kind: MessageKind,
	/// Time that the packet was sent.
	send_time: u64,
	/// Any additional flags that were passed.
	flags: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageKind {
	/// Command being sent.
	Request(Command),
	/// Response to a previously-sent command.
	Response(u64, CommandResponse),
	/// Acknowledges a previously-sent packet.
	Ack(u64)
}