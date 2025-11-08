use crate::gpio_addresses::*;
use crate::gpio_mode_types::GPIOError;

pub fn get_selection_pointer(pin: u8) -> Result<(*mut u32, u8), GPIOError> {
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

pub fn get_low_write_pointer(pin: u8) -> Result<(*mut u32, u8), GPIOError> {
    match pin {
        0..=31 => Ok((GPIO_SET_LOW_0, pin)),
        32..=53 => Ok((GPIO_SET_LOW_1, pin - 32)),
        _ => Err(GPIOError::NonExistentPin),
    }
}

pub fn get_high_write_pointer(pin: u8) -> Result<(*mut u32, u8), GPIOError> {
    match pin {
        0..=31 => Ok((GPIO_SET_HIGH_0, pin)),
        32..=53 => Ok((GPIO_SET_HIGH_1, pin - 32)),
        _ => Err(GPIOError::NonExistentPin),
    }
}

pub fn get_read_pointer(pin: u8) -> Result<(*mut u32, u8), GPIOError> {
    match pin {
        0..=31 => Ok((GPIO_GET_LEVEL_0, pin)),
        32..=53 => Ok((GPIO_GET_LEVEL_1, pin - 32)),
        _ => Err(GPIOError::NonExistentPin),
    }
}

pub fn get_set_pull_clock_pointer(pin: u8) -> Result<(*mut u32, u8), GPIOError> {
    match pin {
        0..=31 => Ok((GPIO_PIN_SET_PULL_CLOCK_0, pin)),
        32..=53 => Ok((GPIO_PIN_SET_PULL_CLOCK_1, pin - 32)),
        _ => Err(GPIOError::NonExistentPin),
    }
}
