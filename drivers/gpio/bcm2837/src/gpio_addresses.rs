const GPIO_FUNCTION_SELECT_0: *mut u32 = 0x7E20_0000 as *mut u32; // GPSEL0
const GPIO_FUNCTION_SELECT_1: *mut u32 = 0x7E20_0004 as *mut u32; // GPSEL1
const GPIO_FUNCTION_SELECT_2: *mut u32 = 0x7E20_0008 as *mut u32; // GPSEL2
const GPIO_FUNCTION_SELECT_3: *mut u32 = 0x7E20_000C as *mut u32; // GPSEL3
const GPIO_FUNCTION_SELECT_4: *mut u32 = 0x7E20_0010 as *mut u32; // GPSEL4
const GPIO_FUNCTION_SELECT_5: *mut u32 = 0x7E20_0014 as *mut u32; // GPSEL 5

const GPIO_SET_HIGH_0: *mut u32 = 0x7E20_001C as *mut u32; // GPSET0
const GPIO_SET_HIGH_1: *mut u32 = 0x7E20_0020 as *mut u32; // GPSET1

const GPIO_SET_LOW_0: *mut u32 = 0x7E20_0028 as *mut u32; // GPCLR0
const GPIO_SET_LOW_1: *mut u32 = 0x7E20_002C as *mut u32; // GPCLR1

const GPIO_GET_LEVEL_0: *mut u32 = 0x7E20_0034 as *mut u32; // GPLEV0
const GPIO_GET_LEVEL_1: *mut u32 = 0x7E20_0038 as *mut u32; // GPLEV1

// TODO: Handle Event Interrupts

const GPIO_PIN_SET_PULL: *mut u32 = 0x7E20_0094 as *mut u32; // GPPUD
const GPIO_PIN_SET_PULL_CLOCK_0: *mut u32 = 0x7E20_0098 as *mut u32; // GPPUDCLK0
const GPIO_PIN_SET_PULL_CLOCK_1: *mut u32 = 0x7E20_0098C as *mut u32; // GPPUDCLK1
