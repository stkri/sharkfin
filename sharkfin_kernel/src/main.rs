#![no_std]
#![no_main]
//! This is the main kernel file.
//! **Do *not* edit this, unless you really know what you are doing.**\
//! You probably want to create an external module.
use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use device::DEVICE;
use device::init_global_device;

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("asm/aarch64/boot.aarch64.s"));

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("asm/aarch64/vector_table.aarch64.s"));

/// Main privileged space runtime of the kernel.\
/// **Do not edit unless you know what you are doing.**
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    init_global_device();

    let mut d = critical_section::with(|cs| DEVICE.borrow(cs).get().unwrap());

    d.run_dsh();
    loop {
        core::hint::spin_loop();
    }
}
/// Kernel panic handler.\
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(mut device) = critical_section::with(|cs| DEVICE.borrow(cs).get()) {
        let uart = &mut device.uart;
        writeln!(uart).unwrap();
        writeln!(uart, "[ !!!!!! ]\tKERNEL PANIC").ok();
        writeln!(uart, "{info}").ok();
    }

    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sync_handler() {
    panic!("Synchronous exception!");
}

#[unsafe(no_mangle)]
pub extern "C" fn irq_handler() {
    panic!("IRQ - not implemented yet!");
}

#[unsafe(no_mangle)]
pub extern "C" fn fiq_handler() {
    panic!("FIQ - unexpected!");
}

#[unsafe(no_mangle)]
pub extern "C" fn serror_handler() {
    panic!("SError - hardware error!");
}
