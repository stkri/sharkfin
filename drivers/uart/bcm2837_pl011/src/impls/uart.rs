pub use crate::uart_types::*;
use driver_gpio_bcm2837::gpio_types::GPIOAlt0;
use crate::uart_addresses::*;
pub use driver_traits::uart::SerialOutput;
use core::fmt::Write;
pub use driver_traits::uart::SerialInput;
pub use driver_traits::uart::SerialFull;

const CLOCK_FREQ: u32 = 48_000_000u32;

impl Write for UART {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            match self.write_byte(c) {
                Ok(_) => {},
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
        unsafe { DATA_REGISTER.write_volatile(byte as u32) };
        Ok(())
    }
}

impl SerialInput for UART {
    type Error = UARTError;
    fn read_byte(&self) -> UARTResult<u8> {
        while unsafe { FLAG_REGISTER.read_volatile() } & 0b1_0000 == 0 {}
        Ok(unsafe { DATA_REGISTER.read_volatile() as u8 })
    }
}

impl SerialFull for UART {}
impl UART {
    pub unsafe fn new(baud_rate: u32) -> Self {
        unsafe { GPIOAlt0::new(14).unwrap() };
        unsafe { GPIOAlt0::new(15).unwrap() };

        unsafe { CONTROL_REGISTER.write_volatile(0) };

        while unsafe { FLAG_REGISTER.read_volatile() } & 0b1000 == 0b1000 {}

        unsafe { LINE_CONTROL_REGISTER.write_volatile(0b0_11_1_0_0_0_0_0) };

        let fractional = CLOCK_FREQ as f32 / (16 * baud_rate) as f32;
        let ibrd = fractional as u32;
        let fbrd= (( fractional - ibrd as f32 ) * 64.0) as u32;

        unsafe { INTEGER_BAUD_RATE_DIVISOR.write_volatile(ibrd) }
        unsafe { FRACTION_BAUD_RATE_DIVISOR.write_volatile(fbrd) }

        unsafe { CONTROL_REGISTER.write_volatile(0b11_0000_0001) };

        UART
    }
}
