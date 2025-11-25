use crate::uart_addresses::{
    CONTROL_REGISTER, DATA_REGISTER, FLAG_REGISTER, FRACTION_BAUD_RATE_DIVISOR,
    INTEGER_BAUD_RATE_DIVISOR, LINE_CONTROL_REGISTER,
};
pub use crate::uart_types::*;
use core::fmt::Write;
use driver_gpio_bcm2837::gpio_types::GPIOAlt0;
pub use driver_traits::uart::SerialFull;
pub use driver_traits::uart::SerialInput;
pub use driver_traits::uart::SerialOutput;

const CLOCK_FREQ: u32 = 48_000_000u32;

impl Write for UART {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            if c == b'\n' {
                match self.write_byte(b'\r') {
                    Ok(()) => {}
                    Err(_) => return Err(core::fmt::Error),
                }
            }
            match self.write_byte(c) {
                Ok(()) => {}
                Err(_) => return Err(core::fmt::Error),
            }
        }
        Ok(())
    }
}
impl SerialOutput for UART {
    type Error = UARTError;
    fn write_byte(&mut self, byte: u8) -> UARTResult<()> {
        while unsafe { FLAG_REGISTER.read_volatile() } & 0b10_0000 != 0 {}
        unsafe { DATA_REGISTER.write_volatile(u32::from(byte)) };
        Ok(())
    }
}

impl SerialInput for UART {
    type Error = UARTError;
    #[allow(clippy::cast_possible_truncation)]
    fn read_byte(&self) -> UARTResult<u8> {
        while unsafe { FLAG_REGISTER.read_volatile() } & 0b1_0000 != 0 {}
        Ok(unsafe { DATA_REGISTER.read_volatile() as u8 })
    }
}

impl SerialFull for UART {}
impl UART {
    /// Creates a new UART instance with the specified baud rate.
    ///
    /// # Safety
    /// This function accesses hardware registers directly. The caller must ensure
    /// that the UART hardware is present and not being used elsewhere.
    ///
    /// # Panics
    /// Panics if GPIO pins 14 or 15 cannot be configured for alternate function 0.
    #[must_use]
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub unsafe fn new(baud_rate: u32) -> Self {
        unsafe { GPIOAlt0::new(14).unwrap() };
        unsafe { GPIOAlt0::new(15).unwrap() };

        unsafe { CONTROL_REGISTER.write_volatile(0) };

        unsafe { LINE_CONTROL_REGISTER.write_volatile(0b0111_0000) };

        let fractional = CLOCK_FREQ as f32 / (16 * baud_rate) as f32;
        let ibrd = fractional as u32;
        let fbrd = ((fractional - ibrd as f32) * 64.0) as u32;

        unsafe { INTEGER_BAUD_RATE_DIVISOR.write_volatile(ibrd) }
        unsafe { FRACTION_BAUD_RATE_DIVISOR.write_volatile(fbrd) }

        unsafe { CONTROL_REGISTER.write_volatile(0b11_0000_0001) };

        UART
    }
}
