#![no_std]
#![no_main]
//! This is the main kernel file.
//! **Do *not* edit this, unless you really know what you are doing.**\
//! You probably want to create an external module.
use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use device::Device;
use uart::uart_types::UART;

const BAUD_RATE: u32 = 115_200u32;

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("asm/aarch64/boot.aarch64.s"));
/// Main privileged space runtime of the kernel.\
/// **Do not edit unless you know what you are doing.**
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut d = Device::new();
    d.run_dsh();
    loop {}
}
/// Kernel panic handler.\
/// **Do not edit unless you know what you are doing.**
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut u = unsafe { UART::new(BAUD_RATE) };
    writeln!(u, "").unwrap();

    writeln!(u, "[ !!!!!! ]\tKERNEL PANIC").ok();
    writeln!(u, "{}", info).ok();
    loop {}
}
