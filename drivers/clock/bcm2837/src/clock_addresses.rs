use core::ptr::NonNull;

pub const CLOCK_COUNT_LOW: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F00_3004 as *mut u32) };

pub const CLOCK_COUNT_HIGH: NonNull<u32> =
    unsafe { NonNull::new_unchecked(0x3F00_3008 as *mut u32) };
