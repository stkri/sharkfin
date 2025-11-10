//! MMIO mappings for the BCM2837.\
//! This library uses NonNulls.

use core::ptr::NonNull;

/// Function selection pointer for pins 0 through 9.
/// Write the correct three bit configuration for the desired mode starting at
/// bit `pin * 3`.
///
/// # Modes
/// - `000`: Input
/// - `001`: Output
/// - `100`: Alt0
/// - `101`: Alt1
/// - `110`: Alt2
/// - `111`: Alt3
/// - `011`: Alt4
/// - `010`: Alt5
///
/// For more info on each mode, check the corresponding GPIO type.
pub const GPIO_FUNCTION_SELECT_0: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_0000 as *mut u32) };

/// Function selection pointer for pins 10 through 19.
/// Write the correct three bit configuration for the desired mode starting at
/// bit `(pin - 10) * 3`.
///
/// # Modes
/// - `000`: Input
/// - `001`: Output
/// - `100`: Alt0
/// - `101`: Alt1
/// - `110`: Alt2
/// - `111`: Alt3
/// - `011`: Alt4
/// - `010`: Alt5
///
/// For more info on each mode, check the corresponding GPIO type.
pub const GPIO_FUNCTION_SELECT_1: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_0004 as *mut u32) };

/// Function selection pointer for pins 20 through 29.
/// Write the correct three bit configuration for the desired mode starting at
/// bit `(pin - 20) * 3`.
///
/// # Modes
/// - `000`: Input
/// - `001`: Output
/// - `100`: Alt0
/// - `101`: Alt1
/// - `110`: Alt2
/// - `111`: Alt3
/// - `011`: Alt4
/// - `010`: Alt5
///
/// For more info on each mode, check the corresponding GPIO type.
pub const GPIO_FUNCTION_SELECT_2: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_0008 as *mut u32) };

/// Function selection pointer for pins 30 through 39.
/// Write the correct three bit configuration for the desired mode starting at
/// bit `(pin - 30) * 3`.
///
/// # Modes
/// - `000`: Input
/// - `001`: Output
/// - `100`: Alt0
/// - `101`: Alt1
/// - `110`: Alt2
/// - `111`: Alt3
/// - `011`: Alt4
/// - `010`: Alt5
///
/// For more info on each mode, check the corresponding GPIO type.
pub const GPIO_FUNCTION_SELECT_3: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_000C as *mut u32) };

/// Function selection pointer for pins 40 through 49.
/// Write the correct three bit configuration for the desired mode starting at
/// bit `(pin - 40) * 3`.
///
/// # Modes
/// - `000`: Input
/// - `001`: Output
/// - `100`: Alt0
/// - `101`: Alt1
/// - `110`: Alt2
/// - `111`: Alt3
/// - `011`: Alt4
/// - `010`: Alt5
///
/// For more info on each mode, check the corresponding GPIO type.
pub const GPIO_FUNCTION_SELECT_4: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_0010 as *mut u32) };

/// Function selection pointer for pins 50 through 53.
/// Write the correct three bit configuration for the desired mode starting at
/// bit `(pin - 50) * 3`.
///
/// # Modes
/// - `000`: Input
/// - `001`: Output
/// - `100`: Alt0
/// - `101`: Alt1
/// - `110`: Alt2
/// - `111`: Alt3
/// - `011`: Alt4
/// - `010`: Alt5
///
/// For more info on each mode, check the corresponding GPIO type.
pub const GPIO_FUNCTION_SELECT_5: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_0014 as *mut u32) };

/// Set high voltage for pins 0 through 31.
/// To set a pin high, set the bit `pin` to 1.
/// All 0s are ignored
pub const GPIO_SET_HIGH_0: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_001C as *mut u32) };

/// Set high voltage for pins 32 through 53.
/// To set a pin high, set the bit `pin - 32` to 1.
/// All 0s are ignored
pub const GPIO_SET_HIGH_1: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_0020 as *mut u32) };

/// Set low voltage for pins 0 through 31.
/// To set a pin high, set the bit `pin` to 1.
/// All 0s are ignored
pub const GPIO_SET_LOW_0: NonNull<u32> = unsafe { NonNull::new_unchecked(0x3F20_0028 as *mut u32) };

/// Set low voltage for pins 32 through 53.
/// To set a pin high, set the bit `pin - 32` to 1.
/// All 0s are ignored
pub const GPIO_SET_LOW_1: NonNull<u32> = unsafe { NonNull::new_unchecked(0x3F20_002C as *mut u32) };

/// Get the voltage levels for pins 0 through 31.
/// Each bit represents a pin, with 1 being high and 0 being low.
pub const GPIO_GET_LEVEL_0: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_0034 as *mut u32) };

/// Get the voltage levels for pins 32 through 53.
/// Each bit represents a pin, with 1 being high and 0 being low.
pub const GPIO_GET_LEVEL_1: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_0038 as *mut u32) };

// TODO: Handle Event Interrupts

/// Select the pull mode. There are three valid options.
/// - `00`: None
/// - `01`: Pull down
/// - `10`: Pull up
///
/// # Usage
/// To be used with GPIO_PIN_SET_PULL_CLOCK in following way:
/// 1. Write to GPIO_PIN_SET_PULL
/// 2. Wait 150 cycles
/// 3. Write to GPIO_PIN_SET_PULL_CLOCK_*
/// 4. Wait 150 cycles
/// 5. Clear GPIO_PIN_SET_PULL
/// 6. Clear GPIO_PIN_SET_PULL_CLOCK_*
pub const GPIO_PIN_SET_PULL: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_0094 as *mut u32) };

/// Select pins to configure pull.
/// If bit `pin` is set to 1, the pull will be changed to what is chosen.
///
/// # Usage
/// To be used with GPIO_PIN_SET_PULL_CLOCK in following way:
/// 1. Write to GPIO_PIN_SET_PULL
/// 2. Wait 150 cycles
/// 3. Write to GPIO_PIN_SET_PULL_CLOCK_*
/// 4. Wait 150 cycles
/// 5. Clear GPIO_PIN_SET_PULL
/// 6. Clear GPIO_PIN_SET_PULL_CLOCK_*
pub const GPIO_PIN_SET_PULL_CLOCK_0: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_0098 as *mut u32) };

/// Select pins to configure pull.
/// If bit `pin - 32` is set to 1, the pull will be changed to what is chosen.
///
/// # Usage
/// To be used with GPIO_PIN_SET_PULL_CLOCK in following way:
/// 1. Write to GPIO_PIN_SET_PULL
/// 2. Wait 150 cycles
/// 3. Write to GPIO_PIN_SET_PULL_CLOCK_*
/// 4. Wait 150 cycles
/// 5. Clear GPIO_PIN_SET_PULL
/// 6. Clear GPIO_PIN_SET_PULL_CLOCK_*
pub const GPIO_PIN_SET_PULL_CLOCK_1: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_009C as *mut u32) };
