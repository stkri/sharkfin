//! Common utility functions for the BCM2837 GPIO.
//! These are intended to be used inside the driver,
//! but may also be used for other purposes.

use crate::gpio_addresses::*;
use crate::gpio_types::GPIOError;
use crate::gpio_types::GPIOPull;
use crate::gpio_types::GPIOResult;
use core::ptr::NonNull;

/// Function which returns he correct `GPFSEL` pointer for the given pin ID.
///
/// # Errors:
/// Returns an error if `pin` is higher than 53.
pub fn get_selection_pointer(pin: u8) -> GPIOResult<(NonNull<u32>, u8)> {
    match pin {
        0..=9 => Ok((GPIO_FUNCTION_SELECT_0, pin * 3)),
        10..=19 => Ok((GPIO_FUNCTION_SELECT_1, (pin - 10) * 3)),
        20..=29 => Ok((GPIO_FUNCTION_SELECT_2, (pin - 20) * 3)),
        30..=39 => Ok((GPIO_FUNCTION_SELECT_3, (pin - 30) * 3)),
        40..=49 => Ok((GPIO_FUNCTION_SELECT_4, (pin - 40) * 3)),
        50..=53 => Ok((GPIO_FUNCTION_SELECT_5, (pin - 50) * 3)),
        _ => Err(GPIOError::NonExistentPin),
    }
}

/// Function which returns he correct `GPCLR` pointer for the given pin ID.
///
/// # Errors:
/// Returns an error if `pin` is higher than 53.
pub fn get_low_write_pointer(pin: u8) -> GPIOResult<(NonNull<u32>, u8)> {
    match pin {
        0..=31 => Ok((GPIO_SET_LOW_0, pin)),
        32..=53 => Ok((GPIO_SET_LOW_1, pin - 32)),
        _ => Err(GPIOError::NonExistentPin),
    }
}

/// Function which returns he correct `GPSET` pointer for the given pin ID.
///
/// # Errors:
/// Returns an error if `pin` is higher than 53.
pub fn get_high_write_pointer(pin: u8) -> GPIOResult<(NonNull<u32>, u8)> {
    match pin {
        0..=31 => Ok((GPIO_SET_HIGH_0, pin)),
        32..=53 => Ok((GPIO_SET_HIGH_1, pin - 32)),
        _ => Err(GPIOError::NonExistentPin),
    }
}

/// Function which returns he correct `GPLEV` pointer for the given pin ID.
///
/// # Errors:
/// Returns an error if `pin` is higher than 53.
pub fn get_read_pointer(pin: u8) -> GPIOResult<(NonNull<u32>, u8)> {
    match pin {
        0..=31 => Ok((GPIO_GET_LEVEL_0, pin)),
        32..=53 => Ok((GPIO_GET_LEVEL_1, pin - 32)),
        _ => Err(GPIOError::NonExistentPin),
    }
}

/// Function which returns he correct `GPPUDCLK` pointer for the given pin ID.
///
/// # Errors:
/// Returns an error if `pin` is higher than 53.
pub fn get_set_pull_clock_pointer(pin: u8) -> GPIOResult<(NonNull<u32>, u8)> {
    match pin {
        0..=31 => Ok((GPIO_PIN_SET_PULL_CLOCK_0, pin)),
        32..=53 => Ok((GPIO_PIN_SET_PULL_CLOCK_1, pin - 32)),
        _ => Err(GPIOError::NonExistentPin),
    }
}

/// Gets the pull state the pin is in after a power up.
///
/// # Safety.
/// This function is marked unsafe because the assumption that it is used right
/// after power up must be true. If it isn't, and pins have already been modified,
/// this doesn't provide any meaningful information.
pub unsafe fn default_pull_after_power_up(pin: u8) -> GPIOResult<GPIOPull> {
    match pin {
        0..=8 | 34..=36 | 46..=53 => Ok(GPIOPull::Up),
        9..=27 | 30..=33 | 37..=43 => Ok(GPIOPull::Down),
        28..=29 | 44..=45 => Ok(GPIOPull::None),
        _ => Err(GPIOError::NonExistentPin),
    }
}
