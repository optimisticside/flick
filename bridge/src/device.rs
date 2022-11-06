use core::fmt;

use crate::message::Message;

/// A bridge device is any device that can transmit and recieve packets.
pub trait Device {
    /// Transmit a packet.
    fn transmit(&self, message: Message) -> Result<(), DeviceError>;
    /// Poll the device for a message. If no message is found, then `Ok(None)` is returned.
    fn poll(&self) -> Result<Option<Message>, DeviceError>;
    /// Send a ping signal to all devices on a given frequency, with a unique identifier and
    /// frequency to communicate on. A device that wants to pair should send a handshake on that
    /// frequency to establish a connection.
    fn ping(&self, id: u64, freq: u64) -> Result<(), DeviceError>;
    /// Retrieve the frequency of the device.
    fn freq(&self) -> Result<u64, DeviceError>;
    /// Set the frequency of the device.
    fn set_freq(&mut self, freq: u64) -> Result<(), DeviceError>;
}

#[derive(Debug, Clone)]
pub enum DeviceError {
    /// Invalid message was provided.
    InvalidMessage,
    /// Invalid frequency was provdied when attempting to change the frequency.
    InvalidFrequency,
    /// Reciever did not acknowledge a message that the transmitter specified needed to be
    /// acknowledged.
    NotAcknowledged,
    /// An error with the physical device occured.
    Physical,
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceError::InvalidFrequency => write!(f, "invalid frequency provided")?,
            DeviceError::InvalidMessage => write!(f, "invalid message provided")?,
            DeviceError::NotAcknowledged => write!(f, "reciever did not acknowledge sent packet")?,
            DeviceError::Physical => write!(f, "physical device error")?,
        };

        Ok(())
    }
}
