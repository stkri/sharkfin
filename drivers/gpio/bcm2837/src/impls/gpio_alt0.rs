use crate::common_functions::default_pull_after_power_up;
use crate::common_functions::get_selection_pointer;
use crate::common_functions::get_set_pull_clock_pointer;
use crate::gpio_addresses::GPIO_PIN_SET_PULL;
pub use crate::gpio_types::GPIOAlt0;
use crate::gpio_types::GPIOError;
use crate::gpio_types::GPIOPull;
use crate::gpio_types::GPIOResult;
pub use driver_traits::gpio_pin::ConfigurablePull;
use kernel_utils::nops::wait_cycles;

impl ConfigurablePull for GPIOAlt0 {
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

impl GPIOAlt0 {
    /// Creates a new GPIO pin configured for alternate function 0.
    ///
    /// # Safety
    /// This function accesses hardware registers directly. The caller must ensure
    /// that the hardware addresses are valid and that the pin is not being used elsewhere.
    ///
    /// # Errors
    /// Returns an error if the pin number is invalid, reserved, or internal.
    pub unsafe fn new(pin_num: u8) -> GPIOResult<Self> {
        let (sel_ptr, mode_bit) = get_selection_pointer(pin_num)?;

        match pin_num {
            16..=17 | 22..=27 | 30..=31 | 33 => return Err(GPIOError::ReservedPinFunction),
            46..=53 => return Err(GPIOError::InternalPinFunction),
            0..=53 => {}
            _ => return Err(GPIOError::NonExistentPin),
        }

        // Safety:
        // We know that sel_ptr is not null.
        let mut current_mode = unsafe { sel_ptr.read_volatile() };
        current_mode &= !(0b111 << mode_bit);
        current_mode |= 0b100 << mode_bit;

        unsafe { sel_ptr.write_volatile(current_mode) };
        let pull = unsafe { default_pull_after_power_up(pin_num)? };

        Ok(Self { pin: pin_num, pull })
    }
}
