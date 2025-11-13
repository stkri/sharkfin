//! Traits for UART Serial interface.

pub trait SerialInput {
    type Error;
    fn read_byte(&self) -> Result<u8, Self::Error>;
}

pub trait SerialOutput {
    type Error;
    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Error>;
}

pub trait SerialFull: SerialInput + SerialOutput {}
