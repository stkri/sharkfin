//! Traits for UART Serial interface.

pub trait SerialInput {
    type Error;
    /// Read a byte from the serial interface.
    ///
    /// # Errors
    /// Fails if the hardware operation fails.
    fn read_byte(&self) -> Result<u8, Self::Error>;
}

pub trait SerialOutput {
    type Error;
    /// Write a byte to the serial interface.
    ///
    /// # Errors
    /// Fails if the hardware operation fails.
    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Error>;
}

pub trait SerialFull: SerialInput + SerialOutput {}
