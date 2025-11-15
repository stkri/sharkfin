//! Collection of types for the BCM2837 GPIO interface.

/// Different hardware error cases.
#[derive(Debug)]
pub enum GPIOError {
    /// The pin was not configured properly with GPFSEL.
    WrongFunction,
    /// The pin has no pull and therefore can't produce defined results.
    NoPull,
    /// The GPIO pin ID exceeds 53.
    NonExistentPin,
    /// The pin cannot be converted to the alternative mode, as it is reserved.
    ReservedPinFunction,
    /// The pin cannot be converted to the alternative mode, as it is internal.
    InternalPinFunction,
    /// The pin cannot be converted to the alternative mode, as it is not defined.
    UndefinedPinFunction,
    /// An unknown read error.
    UnknownReadError,
    /// An unknown write error. This usually causes a bad state.
    UnknownWriteError,
    // TODO: Add more GPIO error cases.
}

pub type GPIOResult<T> = Result<T, GPIOError>;

/// Default pin state as configured by the pull resistors.
#[derive(Default, PartialEq)]
pub enum GPIOPull {
    /// There is no pull. Can't be used other than for Output.
    #[default]
    None,
    /// The pin is set high per default.
    Up,
    /// The pin is set low per default.
    Down,
}

/// Type corresponding to input (`000`).
#[derive(Default)]
pub struct GPIOIn {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to output (`001`).
#[derive(Default)]
pub struct GPIOOut {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT0 (`100`).
/// Must be converted to the proper peripheral type to be used.
///
/// # Alternate functions:
/// - Pin 0: BSC master 0 - data line (SDAO)
/// - Pin 1: BSC master 0 - clock line (SCLO)
/// - Pin 2: BSC master 1 - data line (SDA1)
/// - Pin 3: BSC master 1 - clock line (SCL1)
/// - Pin 4: General purpose clock 0 (GPCLK0)
/// - Pin 5: General purpose clock 1 (GPCLK1)
/// - Pin 6: General purpose clock 2 (GPCLK2)
/// - Pin 7: SPI 0 - chip select 1 (SPIO CE1 N)
/// - Pin 8: SPI 0 - chip select 0 (SPIO CE0 N)
/// - Pin 9: SPI 0 - MISO (SPIO MISO)
/// - Pin 10: SPI 0 - MOSI (SPIO MOSI)
/// - Pin 11: SPI 0 - serial clock (SPIO SCLK)
/// - Pin 12: Pulse width modulator 0 (PWM0)
/// - Pin 13: Pulse width modulator 1 (PWM1)
/// - Pin 14: UART 0 - transmit data (TXD0)
/// - Pin 15: UART 0 - receive data (RXD0)
/// - Pin 16: *reserved*
/// - Pin 17: *reserved*
/// - Pin 18: PCM Audio - clock (PCM CLK)
/// - Pin 19: PCM Audio - frame sync (PCM FS)
/// - Pin 20: PCM Audio - data in (PCM DIN)
/// - Pin 21: PCM Audio - data out (PCM DOUT)
/// - Pin 22: *reserved*
/// - Pin 23: *reserved*
/// - Pin 24: *reserved*
/// - Pin 25: *reserved*
/// - Pin 26: *reserved*
/// - Pin 27: *reserved*
/// - Pin 28: BSC master 0 - data line (SDAO)
/// - Pin 29: BSC master 0 - clock line (SCLO)
/// - Pin 30: *reserved*
/// - Pin 31: *reserved*
/// - Pin 32: General purpose clock 0 (GPCLK0)
/// - Pin 33: *reserved*
/// - Pin 34: General purpose clock 0 (GPCLK0)
/// - Pin 35: SPI 0 - chip select 1 (SPIO CE1 N)
/// - Pin 36: SPI 0 - chip select 0 (SPIO CE0 N)
/// - Pin 37: SPI 0 - MISO (SPIO MISO)
/// - Pin 38: SPI 0 - MOSI (SPIO MOSI)
/// - Pin 39: SPI 0 - serial clock (SPIO SCLK)
/// - Pin 40: Pulse width modulator 0 (PWM0)
/// - Pin 41: Pulse width modulator 1 (PWM1)
/// - Pin 42: General purpose clock 1 (GPCLK1)
/// - Pin 43: General purpose clock 2 (GPCLK2)
/// - Pin 44: General purpose clock 1 (GPCLK1)
/// - Pin 45: Pulse width modulator 1 (PWM1)
/// - Pin 46: *internal*
/// - Pin 47: *internal*
/// - Pin 48: *internal*
/// - Pin 49: *internal*
/// - Pin 50: *internal*
/// - Pin 51: *internal*
/// - Pin 52: *internal*
/// - Pin 53: *internal*
pub struct GPIOAlt0 {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT1 (`101`).
/// Must be converted to the proper peripheral type to be used.
///
/// # Alternate functions:
/// - Pin 0: Secondary memory Address bus (SA5)
/// - Pin 1: Secondary memory Address bus (SA4)
/// - Pin 2: Secondary memory Address bus (SA3)
/// - Pin 3: Secondary memory Address bus (SA2)
/// - Pin 4: Secondary memory Address bus (SA1)
/// - Pin 5: Secondary memory Address bus (SA0)
/// - Pin 6: Secondary memory Controls (SOE_N/SE)
/// - Pin 7: Secondary memory Controls (SWE_N/SRW_N)
/// - Pin 8: Secondary memory data bus (SD0)
/// - Pin 9: Secondary memory data bus (SD1)
/// - Pin 10: Secondary memory data bus (SD2)
/// - Pin 11: Secondary memory data bus (SD3)
/// - Pin 12: Secondary memory data bus (SD4)
/// - Pin 13: Secondary memory data bus (SD5)
/// - Pin 14: Secondary memory data bus (SD6)
/// - Pin 15: Secondary memory data bus (SD7)
/// - Pin 16: Secondary memory data bus (SD8)
/// - Pin 17: Secondary memory data bus (SD9)
/// - Pin 18: Secondary memory data bus (SD10)
/// - Pin 19: Secondary memory data bus (SD11)
/// - Pin 20: Secondary memory data bus (SD12)
/// - Pin 21: Secondary memory data bus (SD13)
/// - Pin 22: Secondary memory data bus (SD14)
/// - Pin 23: Secondary memory data bus (SD15)
/// - Pin 24: Secondary memory data bus (SD16)
/// - Pin 25: Secondary memory data bus (SD17)
/// - Pin 26: *reserved*
/// - Pin 27: *reserved*
/// - Pin 28: Secondary memory Address bus (SA5)
/// - Pin 29: Secondary memory Address bus (SA4)
/// - Pin 30: Secondary memory Address bus (SA3)
/// - Pin 31: Secondary memory Address bus (SA2)
/// - Pin 32: Secondary memory Address bus (SA1)
/// - Pin 33: Secondary memory Address bus (SA0)
/// - Pin 34: Secondary memory Controls (SOE_N/SE)
/// - Pin 35: Secondary memory Controls (SWE_N/SRW_N)
/// - Pin 36: Secondary memory data bus (SD0)
/// - Pin 37: Secondary memory data bus (SD1)
/// - Pin 38: Secondary memory data bus (SD2)
/// - Pin 39: Secondary memory data bus (SD3)
/// - Pin 40: Secondary memory data bus (SD4)
/// - Pin 41: Secondary memory data bus (SD5)
/// - Pin 42: Secondary memory data bus (SD6)
/// - Pin 43: Secondary memory data bus (SD7)
/// - Pin 44: BSC master 0 - data line (SDAO)
/// - Pin 45: BSC master 0 - clock line (SCLO)
/// - Pin 46: *undefined*
/// - Pin 47: *undefined*
/// - Pin 48: *undefined*
/// - Pin 49: *undefined*
/// - Pin 50: *undefined*
/// - Pin 51: *undefined*
/// - Pin 52: *undefined*
/// - Pin 53: *undefined*
pub struct GPIOAlt1 {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT2 (`110`).
/// Must be converted to the proper peripheral type to be used.
///
/// # Alternate functions:
/// - Pin 0: *reserved*
/// - Pin 1: *reserved*
/// - Pin 2: *reserved*
/// - Pin 3: *reserved*
/// - Pin 4: *reserved*
/// - Pin 5: *reserved*
/// - Pin 6: *reserved*
/// - Pin 7: *reserved*
/// - Pin 8: *reserved*
/// - Pin 9: *reserved*
/// - Pin 10: *reserved*
/// - Pin 11: *reserved*
/// - Pin 12: *reserved*
/// - Pin 13: *reserved*
/// - Pin 14: *reserved*
/// - Pin 15: *reserved*
/// - Pin 16: *reserved*
/// - Pin 17: *reserved*
/// - Pin 18: *reserved*
/// - Pin 19: *reserved*
/// - Pin 20: *reserved*
/// - Pin 21: *reserved*
/// - Pin 22: *reserved*
/// - Pin 23: *reserved*
/// - Pin 24: *reserved*
/// - Pin 25: *reserved*
/// - Pin 26: *reserved*
/// - Pin 27: *reserved*
/// - Pin 28: PCM Audio - clock (PCM CLK)
/// - Pin 29: PCM Audio - frame sync (PCM FS)
/// - Pin 30: PCM Audio - data in (PCM DIN)
/// - Pin 31: PCM Audio - data out (PCM DOUT)
/// - Pin 32: *reserved*
/// - Pin 33: *reserved*
/// - Pin 34: *reserved*
/// - Pin 35: *undefined*
/// - Pin 36: UART 0 - transmit data (TXD0)
/// - Pin 37: UART 0 - receive data (RXD0)
/// - Pin 38: UART 0 - clear to send (CTS0)
/// - Pin 39: UART 0 - request to send (RTS0)
/// - Pin 40: *undefined*
/// - Pin 41: *reserved*
/// - Pin 42: *reserved*
/// - Pin 43: *reserved*
/// - Pin 44: BSC master 1 - data line (SDA1)
/// - Pin 45: BSC master 1 - clock line (SCL1)
/// - Pin 46: *undefined*
/// - Pin 47: *undefined*
/// - Pin 48: *undefined*
/// - Pin 49: *undefined*
/// - Pin 50: *undefined*
/// - Pin 51: *undefined*
/// - Pin 52: *undefined*
/// - Pin 53: *undefined*
pub struct GPIOAlt2 {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT3 (`111`).
/// Must be converted to the proper peripheral type to be used.
///
/// # Alternate functions:
/// - Pin 0: *undefined*
/// - Pin 1: *undefined*
/// - Pin 2: *undefined*
/// - Pin 3: *undefined*
/// - Pin 4: *undefined*
/// - Pin 5: *undefined*
/// - Pin 6: *undefined*
/// - Pin 7: *undefined*
/// - Pin 8: *undefined*
/// - Pin 9: *undefined*
/// - Pin 10: *undefined*
/// - Pin 11: *undefined*
/// - Pin 12: *undefined*
/// - Pin 13: *undefined*
/// - Pin 14: *undefined*
/// - Pin 15: *undefined*
/// - Pin 16: UART 0 - clear to send (CTS0)
/// - Pin 17: UART 0 - request to send (RTS0)
/// - Pin 18: BSC Slave - data/SPI Slave - MOSI (BSCSL SDA/MOSI)
/// - Pin 19: BSC Slave - clock/SPI Slave - SCLK (BSCSL SCL/SCLK)
/// - Pin 20: SPI Slave - MISO (BSCSL/MISO)
/// - Pin 21: SPI Slave - CSn (BSCSL/CEN)
/// - Pin 22: SD Host - clock (SD1 CLK)
/// - Pin 23: SD Host - command (SD1 CMD)
/// - Pin 24: SD Host - data 0 (SD1 DAT0)
/// - Pin 25: SD Host - data 1 (SD1 DAT1)
/// - Pin 26: SD Host - data 2 (SD1 DAT2)
/// - Pin 27: SD Host - data 3 (SD1 DAT3)
/// - Pin 28: *reserved*
/// - Pin 29: *reserved*
/// - Pin 30: UART 0 - clear to send (CTS0)
/// - Pin 31: UART 0 - request to send (RTS0)
/// - Pin 32: UART 0 - transmit data (TXD0)
/// - Pin 33: UART 0 - receive data (RXD0)
/// - Pin 34: *reserved*
/// - Pin 35: *reserved*
/// - Pin 36: *reserved*
/// - Pin 37: *reserved*
/// - Pin 38: *reserved*
/// - Pin 39: *reserved*
/// - Pin 40: *reserved*
/// - Pin 41: *reserved*
/// - Pin 42: *reserved*
/// - Pin 43: *reserved*
/// - Pin 44: *reserved*
/// - Pin 45: *reserved*
/// - Pin 46: *undefined*
/// - Pin 47: *undefined*
/// - Pin 48: *undefined*
/// - Pin 49: *undefined*
/// - Pin 50: *undefined*
/// - Pin 51: *undefined*
/// - Pin 52: *undefined*
/// - Pin 53: *undefined*
pub struct GPIOAlt3 {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT4 (`000`).
/// Must be converted to the proper peripheral type to be used.
///
/// # Alternate functions:
/// - Pin 0: *undefined*
/// - Pin 1: *undefined*
/// - Pin 2: *undefined*
/// - Pin 3: *undefined*
/// - Pin 4: *undefined*
/// - Pin 5: *undefined*
/// - Pin 6: *undefined*
/// - Pin 7: *undefined*
/// - Pin 8: *undefined*
/// - Pin 9: *undefined*
/// - Pin 10: *undefined*
/// - Pin 11: *undefined*
/// - Pin 12: *undefined*
/// - Pin 13: *undefined*
/// - Pin 14: *undefined*
/// - Pin 15: *undefined*
/// - Pin 16: SPI 1 - chip select 2 (SPI1 CE2 N)
/// - Pin 17: SPI 1 - chip select 1 (SPI1 CE1 N)
/// - Pin 18: SPI 1 - chip select 0 (SPI1 CE0 N)
/// - Pin 19: SPI 1 - MISO (SPI1 MISO)
/// - Pin 20: SPI 1 - MOSI (SPI1 MOSI)
/// - Pin 21: SPI 1 - serial clock (SPI1 SCLK)
/// - Pin 22: ARM JTAG reset (ARM TRST)
/// - Pin 23: ARM JTAG return clock (ARM RTCK)
/// - Pin 24: ARM JTAG data out (ARM TDO)
/// - Pin 25: ARM JTAG clock (ARM TCK)
/// - Pin 26: ARM JTAG data in (ARM TDI)
/// - Pin 27: ARM JTAG mode select (ARM TMS)
/// - Pin 28: *undefined*
/// - Pin 29: *undefined*
/// - Pin 30: *undefined*
/// - Pin 31: *undefined*
/// - Pin 32: *undefined*
/// - Pin 33: *undefined*
/// - Pin 34: *undefined*
/// - Pin 35: *undefined*
/// - Pin 36: *undefined*
/// - Pin 37: *undefined*
/// - Pin 38: *undefined*
/// - Pin 39: *undefined*
/// - Pin 40: SPI 2 - MISO (SPI2 MISO)
/// - Pin 41: SPI 2 - MOSI (SPI2 MOSI)
/// - Pin 42: SPI 2 - serial clock (SPI2 SCLK)
/// - Pin 43: SPI 2 - chip select 0 (SPI2 CE0 N)
/// - Pin 44: SPI 2 - chip select 1 (SPI2 CE1 N)
/// - Pin 45: SPI 2 - chip select 2 (SPI2 CE2 N)
/// - Pin 46: *undefined*
/// - Pin 47: *undefined*
/// - Pin 48: *undefined*
/// - Pin 49: *undefined*
/// - Pin 50: *undefined*
/// - Pin 51: *undefined*
/// - Pin 52: *undefined*
/// - Pin 53: *undefined*
pub struct GPIOAlt4 {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT5 (`001`).
/// Must be converted to the proper peripheral type to be used.
///
/// # Alternate functions:
/// - Pin 0: *undefined*
/// - Pin 1: *undefined*
/// - Pin 2: *undefined*
/// - Pin 3: *undefined*
/// - Pin 4: ARM JTAG data in (ARM TDI)
/// - Pin 5: ARM JTAG data out (ARM TDO)
/// - Pin 6: ARM JTAG return clock (ARM_RTCK)
/// - Pin 7: *undefined*
/// - Pin 8: *undefined*
/// - Pin 9: *undefined*
/// - Pin 10: *undefined*
/// - Pin 11: *undefined*
/// - Pin 12: ARM JTAG mode select (ARM TMS)
/// - Pin 13: ARM JTAG clock (ARM_TCK)
/// - Pin 14: UART 1 - transmit data (TXD1)
/// - Pin 15: UART 1 - receive data (RXD1)
/// - Pin 16: UART 1 - request to send (RTS1)
/// - Pin 17: UART 1 - request to send (RTS1)
/// - Pin 18: Pulse width modulator 0 (PWM0)
/// - Pin 19: Pulse width modulator 1 (PWM1)
/// - Pin 20: General purpose clock 0 (GPCLK0)
/// - Pin 21: General purpose clock 1 (GPCLK1)
/// - Pin 22: *undefined*
/// - Pin 23: *undefined*
/// - Pin 24: *undefined*
/// - Pin 25: *undefined*
/// - Pin 26: *undefined*
/// - Pin 27: *undefined*
/// - Pin 28: *undefined*
/// - Pin 29: *undefined*
/// - Pin 30: UART 1 - clear to send (CTS1)
/// - Pin 31: UART 1 - request to send (RTS1)
/// - Pin 32: UART 1 - transmit data (TXD1)
/// - Pin 33: UART 1 - receive data (RXD1)
/// - Pin 34: *undefined*
/// - Pin 35: *undefined*
/// - Pin 36: *undefined*
/// - Pin 37: *undefined*
/// - Pin 38: *undefined*
/// - Pin 39: *undefined*
/// - Pin 40: UART 1 - transmit data (TXD1)
/// - Pin 41: UART 1 - receive data (RXD1)
/// - Pin 42: UART 1 - request to send (RTS1)
/// - Pin 43: UART 1 - clear to send (CTS1)
/// - Pin 44: *undefined*
/// - Pin 45: *undefined*
/// - Pin 46: *undefined*
/// - Pin 47: *undefined*
/// - Pin 48: *undefined*
/// - Pin 49: *undefined*
/// - Pin 50: *undefined*
/// - Pin 51: *undefined*
/// - Pin 52: *undefined*
/// - Pin 53: *undefined*
pub struct GPIOAlt5 {
    pub pin: u8,
    pub pull: GPIOPull,
}
