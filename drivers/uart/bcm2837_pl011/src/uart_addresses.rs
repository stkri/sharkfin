//! MMIO Mappings for UART registers on the BCM2837

use core::ptr::NonNull;

pub const DATA_REGISTER: NonNull<u32> = unsafe { NonNull::new_unchecked(0x3F20_1000 as *mut u32) };
pub const FLAG_REGISTER: NonNull<u32> = unsafe { NonNull::new_unchecked(0x3F20_1018 as *mut u32) };
pub const INTEGER_BAUD_RATE_DIVISOR: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_1024 as *mut u32) };
pub const FRACTION_BAUD_RATE_DIVISOR: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_1028 as *mut u32) };
pub const LINE_CONTROL_REGISTER: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_102C as *mut u32) };
pub const CONTROL_REGISTER: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F20_1030 as *mut u32) };
