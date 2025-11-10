#![no_std]
#![no_main]
//! This is the main kernel file.
//! **Do *not* edit this, unless you really know what you are doing.**\
//! You probably want to create an external module.
use core::arch::global_asm;
use core::panic::PanicInfo;
use gpio::gpio_types::GPIOIn;
use gpio::gpio_types::GPIOOut;
use gpio::gpio_types::GPIOPull;
use gpio::impls::gpio_in::InputPin;
use gpio::impls::gpio_out::{OutputPin, StatefulOutputPin};
use gpio::impls::gpio_in::ConfigurablePull;
use kernel_utils::nops::wait_cycles;

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("asm/aarch64/boot.aarch64.s"));
/// Main privileged space runtime of the kernel.\
/// **Do not edit unless you know what you are doing.**
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut led = match unsafe { GPIOOut::new(29) } {
        Ok(pin) => pin,
        Err(_) => panic!("GPIO initialization failed"),
    };
    let mut thing = unsafe { GPIOIn::new(17).unwrap() };
    thing.set_pull(GPIOPull::Down).unwrap();
    wait_cycles(50_000);
    if thing.is_high().unwrap() {
        loop {
            led.toggle().unwrap();
            wait_cycles(50_000);
        }
    }
    loop {}
}
/// Kernel panic handler.\
/// **Do not edit unless you know what you are doing.**
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
