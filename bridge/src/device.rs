use core::fmt;

use crate::packet::Packet;

/// A bridge device is any device that can transmit and recieve packets.
pub trait Device {
    /// Transmit a packet.
    fn transmit(&self, packet: Packet) -> Result<(), DeviceError>;
    /// Poll the device for a packet. If no packet is found, then `Ok(None)` is returned.
    fn poll(&self) -> Result<Option<Packet>, DeviceError>;
}

#[derive(Debug, Clone)]
pub enum DeviceError {
    /// Reciever did not acknowledge a packets that the transmitter specified needed to be
    /// acknowledged.
    NotAcknowledged,
    /// An error with the physical device occured.
    Physical,
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceError::NotAcknowledged => write!(f, "reciever did not acknowledge sent packet")?,
            DeviceError::Physical => write!(f, "physical device error")?,
        };

        Ok(())
    }
}
