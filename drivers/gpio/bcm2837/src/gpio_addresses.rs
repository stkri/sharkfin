pub const GPIO_FUNCTION_SELECT_0: *mut u32 = 0x3F20_0000 as *mut u32; // GPFSEL0
pub const GPIO_FUNCTION_SELECT_1: *mut u32 = 0x3F20_0004 as *mut u32; // GPFSEL1
pub const GPIO_FUNCTION_SELECT_2: *mut u32 = 0x3F20_0008 as *mut u32; // GPFSEL2
pub const GPIO_FUNCTION_SELECT_3: *mut u32 = 0x3F20_000C as *mut u32; // GPFSEL3
pub const GPIO_FUNCTION_SELECT_4: *mut u32 = 0x3F20_0010 as *mut u32; // GPFSEL4
pub const GPIO_FUNCTION_SELECT_5: *mut u32 = 0x3F20_0014 as *mut u32; // GPFSEL 5

pub const GPIO_SET_HIGH_0: *mut u32 = 0x3F20_001C as *mut u32; // GPSET0
pub const GPIO_SET_HIGH_1: *mut u32 = 0x3F20_0020 as *mut u32; // GPSET1

pub const GPIO_SET_LOW_0: *mut u32 = 0x3F20_0028 as *mut u32; // GPCLR0
pub const GPIO_SET_LOW_1: *mut u32 = 0x3F20_002C as *mut u32; // GPCLR1

pub const GPIO_GET_LEVEL_0: *mut u32 = 0x3F20_0034 as *mut u32; // GPLEV0
pub const GPIO_GET_LEVEL_1: *mut u32 = 0x3F20_0038 as *mut u32; // GPLEV1

// TODO: Handle Event Interrupts

pub const GPIO_PIN_SET_PULL: *mut u32 = 0x3F20_0094 as *mut u32; // GPPUD
pub const GPIO_PIN_SET_PULL_CLOCK_0: *mut u32 = 0x3F20_0098 as *mut u32; // GPPUDCLK0
pub const GPIO_PIN_SET_PULL_CLOCK_1: *mut u32 = 0x3F20_009C as *mut u32; // GPPUDCLK1
