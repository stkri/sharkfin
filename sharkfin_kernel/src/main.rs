#![no_std]
#![no_main]
//! This is the main kernel file.
//! **Do *not* edit this, unless you really know what you are doing.**\
//! You probably want to create an external module.
use core::arch::global_asm;
use core::panic::PanicInfo;
use uart::uart_types::UART;
use core::fmt::Write;
use uart::impls::uart::SerialInput;
use kernel_utils::nops::wait_cycles;

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("asm/aarch64/boot.aarch64.s"));
/// Main privileged space runtime of the kernel.\
/// **Do not edit unless you know what you are doing.**
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut u = unsafe { UART::new(115200) };

    loop {
        let c = u.read_byte().unwrap() as char;
        match c {
            '\0' => {},
            c => writeln!(u, "{}", c).unwrap(),
        }
        wait_cycles(1000);
    }
}
/// Kernel panic handler.\
/// **Do not edit unless you know what you are doing.**
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut u = unsafe { UART::new(115200) };
    writeln!(u, "{}", info).ok();
    loop {}
}
