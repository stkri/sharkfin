//! These are the implementations for `GPIOIn`

use crate::common_functions::get_read_pointer;
use crate::common_functions::get_selection_pointer;
use crate::common_functions::{default_pull_after_power_up, get_set_pull_clock_pointer};
use crate::gpio_addresses::GPIO_PIN_SET_PULL;
pub use crate::gpio_types::*;
pub use driver_traits::gpio_pin::ConfigurablePull;
pub use driver_traits::gpio_pin::InputPin;
use kernel_utils::nops::wait_cycles;

impl InputPin for GPIOIn {
    type Error = GPIOError;

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_high()?)
    }
    fn is_high(&self) -> Result<bool, Self::Error> {
        if self.pull == GPIOPull::None {
            return Err(GPIOError::NoPull);
        }
        let (read_ptr, read_bit) = get_read_pointer(self.pin)?;

        // Safety:
        // We know that read_ptr is not null
        let current_state = unsafe { read_ptr.read_volatile() };
        match (current_state >> read_bit) & 1 {
            0b0 => Ok(false),
            0b1 => Ok(true),
            _ => Err(GPIOError::UnknownReadError),
        }
    }
}

impl ConfigurablePull for GPIOIn {
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

        self.pull = pull;

        Ok(())
    }
}

impl TryFrom<GPIOOut> for GPIOIn {
    type Error = GPIOError;
    fn try_from(gpio_out: GPIOOut) -> GPIOResult<Self> {
        let (sel_ptr, mode_bit) = get_selection_pointer(gpio_out.pin)?;

        // Safety:
        // We know that sel_ptr is not null.
        let mut current_mode = unsafe { sel_ptr.read_volatile() };
        current_mode &= !(0b111 << mode_bit);
        current_mode |= 0b000 << mode_bit;

        unsafe { sel_ptr.write_volatile(current_mode) };
        Ok(Self {
            pin: gpio_out.pin,
            pull: gpio_out.pull,
        })
    }
}

impl TryFrom<GPIOAlt0> for GPIOIn {
    type Error = GPIOError;
    fn try_from(gpio_alt0: GPIOAlt0) -> GPIOResult<Self> {
        let (sel_ptr, mode_bit) = get_selection_pointer(gpio_alt0.pin)?;

        // Safety:
        // We know that sel_ptr is not null.
        let mut current_mode = unsafe { sel_ptr.read_volatile() };
        current_mode &= !(0b111 << mode_bit);
        current_mode |= 0b000 << mode_bit;

        unsafe { sel_ptr.write_volatile(current_mode) };
        Ok(Self {
            pin: gpio_alt0.pin,
            pull: gpio_alt0.pull,
        })
    }
}

impl TryFrom<GPIOAlt1> for GPIOIn {
    type Error = GPIOError;
    fn try_from(gpio_alt1: GPIOAlt1) -> GPIOResult<Self> {
        let (sel_ptr, mode_bit) = get_selection_pointer(gpio_alt1.pin)?;

        // Safety:
        // We know that sel_ptr is not null.
        let mut current_mode = unsafe { sel_ptr.read_volatile() };
        current_mode &= !(0b111 << mode_bit);
        current_mode |= 0b000 << mode_bit;

        unsafe { sel_ptr.write_volatile(current_mode) };
        Ok(Self {
            pin: gpio_alt1.pin,
            pull: gpio_alt1.pull,
        })
    }
}

impl TryFrom<GPIOAlt2> for GPIOIn {
    type Error = GPIOError;
    fn try_from(gpio_alt2: GPIOAlt2) -> GPIOResult<Self> {
        let (sel_ptr, mode_bit) = get_selection_pointer(gpio_alt2.pin)?;

        // Safety:
        // We know that sel_ptr is not null.
        let mut current_mode = unsafe { sel_ptr.read_volatile() };
        current_mode &= !(0b111 << mode_bit);
        current_mode |= 0b000 << mode_bit;

        unsafe { sel_ptr.write_volatile(current_mode) };
        Ok(Self {
            pin: gpio_alt2.pin,
            pull: gpio_alt2.pull,
        })
    }
}

impl TryFrom<GPIOAlt3> for GPIOIn {
    type Error = GPIOError;
    fn try_from(gpio_alt3: GPIOAlt3) -> GPIOResult<Self> {
        let (sel_ptr, mode_bit) = get_selection_pointer(gpio_alt3.pin)?;

        // Safety:
        // We know that sel_ptr is not null.
        let mut current_mode = unsafe { sel_ptr.read_volatile() };
        current_mode &= !(0b111 << mode_bit);
        current_mode |= 0b000 << mode_bit;

        unsafe { sel_ptr.write_volatile(current_mode) };
        Ok(Self {
            pin: gpio_alt3.pin,
            pull: gpio_alt3.pull,
        })
    }
}

impl TryFrom<GPIOAlt4> for GPIOIn {
    type Error = GPIOError;
    fn try_from(gpio_alt4: GPIOAlt4) -> GPIOResult<Self> {
        let (sel_ptr, mode_bit) = get_selection_pointer(gpio_alt4.pin)?;

        // Safety:
        // We know that sel_ptr is not null.
        let mut current_mode = unsafe { sel_ptr.read_volatile() };
        current_mode &= !(0b111 << mode_bit);
        current_mode |= 0b000 << mode_bit;

        unsafe { sel_ptr.write_volatile(current_mode) };
        Ok(Self {
            pin: gpio_alt4.pin,
            pull: gpio_alt4.pull,
        })
    }
}

impl TryFrom<GPIOAlt5> for GPIOIn {
    type Error = GPIOError;
    fn try_from(gpio_alt5: GPIOAlt5) -> GPIOResult<Self> {
        let (sel_ptr, mode_bit) = get_selection_pointer(gpio_alt5.pin)?;

        // Safety:
        // We know that sel_ptr is not null.
        let mut current_mode = unsafe { sel_ptr.read_volatile() };
        current_mode &= !(0b111 << mode_bit);
        current_mode |= 0b000 << mode_bit;

        unsafe { sel_ptr.write_volatile(current_mode) };
        Ok(Self {
            pin: gpio_alt5.pin,
            pull: gpio_alt5.pull,
        })
    }
}

impl GPIOIn {
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
        current_mode |= 0b000 << mode_bit;

        unsafe { sel_ptr.write_volatile(current_mode) };
        let pull = unsafe { default_pull_after_power_up(pin_num)? };

        Ok(Self { pin: pin_num, pull })
    }
}
