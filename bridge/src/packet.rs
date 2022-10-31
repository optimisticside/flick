use serde::{Deserialize, Serialize};

/// A packet is the basis for all communication operations that can be done on the bridge. Packets
/// contain the data to be sent, along with other metadata.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Packet;
