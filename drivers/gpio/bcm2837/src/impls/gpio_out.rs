use crate::common_functions::default_pull_after_power_up;
use crate::common_functions::get_high_write_pointer;
use crate::common_functions::get_low_write_pointer;
use crate::common_functions::get_read_pointer;
use crate::common_functions::get_selection_pointer;
use crate::common_functions::get_set_pull_clock_pointer;
use crate::gpio_addresses::GPIO_PIN_SET_PULL;
pub use crate::gpio_types::GPIOError;
pub use crate::gpio_types::GPIOOut;
pub use crate::gpio_types::GPIOPull;
use crate::gpio_types::GPIOResult;
pub use driver_traits::gpio_pin::ConfigurablePull;
pub use driver_traits::gpio_pin::OutputPin;
pub use driver_traits::gpio_pin::StatefulOutputPin;
use kernel_utils::nops::wait_cycles;

impl OutputPin for GPIOOut {
    type Error = GPIOError;
    fn set_low(&mut self) -> GPIOResult<()> {
        let (write_ptr, write_bit) = get_low_write_pointer(self.pin)?;
        let write_num: u32 = 0b1 << write_bit;
        // Safety:
        // We know that write_ptr is not null.
        unsafe { write_ptr.write_volatile(write_num) };

        match self.is_set_low() {
            Ok(true) => Ok(()),
            Ok(false) => Err(GPIOError::UnknownError),
            Err(e) => Err(e),
        }
    }

    fn set_high(&mut self) -> GPIOResult<()> {
        let (write_ptr, write_bit) = get_high_write_pointer(self.pin)?;
        let write_num: u32 = 0b1 << write_bit;
        // Safety:
        // We know that write_ptr is not null.
        unsafe { write_ptr.write_volatile(write_num) };

        match self.is_set_high() {
            Ok(true) => Ok(()),
            Ok(false) => Err(GPIOError::UnknownError),
            Err(e) => Err(e),
        }
    }
}

impl ConfigurablePull for GPIOOut {
    type Error = GPIOError;
    type Pull = GPIOPull;

    fn set_pull(&mut self, pull: Self::Pull) -> GPIOResult<()> {
        let pull_mode_bits = match pull {
            GPIOPull::None => 0b00,
            GPIOPull::Down => 0b01,
            GPIOPull::Up => 0b10,
        };
        let (pull_ptr, bit) = get_set_pull_clock_pointer(self.pin)?;

        // Safety:
        // We know that GPIO_PIN_SET_PTR is not null.
        unsafe { GPIO_PIN_SET_PULL.write_volatile(pull_mode_bits) };
        // According to the datasheet, it is necessary to wait 150 cycles after the pointer writes.
        wait_cycles(150);

        let pin_num = 1 << bit;
        unsafe { pull_ptr.write_volatile(pin_num) };
        wait_cycles(150);

        unsafe { GPIO_PIN_SET_PULL.write_volatile(0) };
        unsafe { pull_ptr.write_volatile(0) };

        Ok(())
    }
}

impl StatefulOutputPin for GPIOOut {
    fn is_set_low(&self) -> GPIOResult<bool> {
        Ok(!self.is_set_high()?)
    }

    fn is_set_high(&self) -> GPIOResult<bool> {
        let (read_ptr, read_bit) = get_read_pointer(self.pin)?;

        // Safety:
        // We know that read_ptr is not null
        let current_state = unsafe { read_ptr.read_volatile() };
        match (current_state >> read_bit) & 1 {
            0b0 => Ok(false),
            0b1 => Ok(true),
            _ => Err(GPIOError::UnknownError),
        }
    }

    fn toggle(&mut self) -> GPIOResult<()> {
        match self.is_set_low() {
            Ok(true) => self.set_high(),
            Ok(false) => self.set_low(),
            Err(e) => Err(e),
        }
    }
}

impl GPIOOut {
    /// Create a new GPIO Output pin object
    ///
    /// # Errors
    /// Fails if the pin is not valid.
    ///
    /// # Safety
    /// Provides no guarantees that the pin isn't used elsewhere.
    /// Because of how it is written, having two variables share a pin
    /// may easily cause Undefined Behaviour.
    pub unsafe fn new(pin_num: u8) -> GPIOResult<Self> {
        let (sel_ptr, mode_bit) = get_selection_pointer(pin_num)?;

        // Safety:
        // We know that sel_ptr is not null.
        let mut current_mode = unsafe { sel_ptr.read_volatile() };
        current_mode &= !(0b111 << mode_bit);
        current_mode |= 0b001 << mode_bit;

        unsafe { sel_ptr.write_volatile(current_mode) };
        let pull = unsafe { default_pull_after_power_up(pin_num)? };

        Ok(Self { pin: pin_num, pull })
    }
}
