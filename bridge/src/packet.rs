use serde::{Deserialize, Serialize};
use crate::command::{Command, CommandResponse};

bitflags::bitflags! {
	/// Flags for packets. See [`Packet::flags`].
	pub struct PacketFlags: u8 {
	}
}

/// A packet is the basis for all communication operations that can be done on the bridge. Packets
/// contain the data to be sent, along with other metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Packet {
	/// Unique identifier used to reference the packet from other data, such as acknowledgement
	/// packets.
	id: u64,
	/// Data specific to the type of packet.
	kind: PacketKind,
	/// Time that the packet was sent.
	send_time: u64,
	/// Any additional flags that were passed.
	flags: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PacketKind {
	/// Command being sent.
	Request(Command),
	/// Response to a previously-sent command.
	Response(u64, CommandResponse),
	/// Acknowledges a previously-sent packet.
	Ack(u64)
}