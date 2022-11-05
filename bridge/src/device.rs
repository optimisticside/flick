use core::fmt;

use crate::message::Message;

/// A bridge device is any device that can transmit and recieve packets.
pub trait Device {
    /// Transmit a packet.
    fn transmit(&self, message: Message) -> Result<(), DeviceError>;
    /// Poll the device for a message. If no message is found, then `Ok(None)` is returned.
    fn poll(&self) -> Result<Option<Message>, DeviceError>;
}

#[derive(Debug, Clone)]
pub enum DeviceError {
    /// Reciever did not acknowledge a message that the transmitter specified needed to be
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
