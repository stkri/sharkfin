#![no_std]
#![no_main]
//! This is the main kernel file.
//! **Do *not* edit this, unless you really know what you are doing.**\
//! You probably want to create an external module.
use core::arch::asm;
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
     setup_page_tables();
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

fn setup_page_tables() {
    unsafe extern "C" {
        static __page_tables_start: u8;
        static __page_tables_end: u8;
        static __heap_start: u8;
    }
    let page_tables_start = unsafe { &__page_tables_start as *const u8 as usize };
    let page_tables_end = unsafe { &__page_tables_end as *const u8 as usize };

    let size = page_tables_end - page_tables_start;

    let page_tables =
        unsafe { core::slice::from_raw_parts_mut(page_tables_start as *mut u64, size / 8) };

    page_tables.fill(0);

    page_tables[0] = (page_tables_start as u64 + 0x1000) | 0b11;
    page_tables[512] = (page_tables_start as u64 + 0x2000) | 0b11;
    for i in 0..512 {
        page_tables[1024 + i] = (page_tables_start as u64 + (i as u64 * 0x1000 + 0x3000)) | 0b11;
    }
    for table in 0..512 {
        for page in 0..512 {
            let index = (table * 512 + 1536) + page;
            let phys_addr = (0x200000 * table + page * 0x1000) as u64;

            let flags = if phys_addr >= 0x3F000000 && phys_addr < 0x40000000 {
                0b11 | (1 << 10) | (3 << 8) | (1 << 2)
            } else {
                0b11 | (1 << 10) | (3 << 8)
            };

            page_tables[index] = phys_addr | flags;
        }
    }


    const MAIR_VALUE: u64 = (0xFF << 0) | (0x00 << 8);

    const TCR_VALUE: u64 =
        (16 << 0) |
        (0b01 << 8) |
        (0b01 << 10) |
        (0b11 << 12) |
        (0b010 << 32);


    unsafe {
        asm!(
            "msr mair_el1, {}",
            in(reg) MAIR_VALUE,
        );

        asm!(
            "msr tcr_el1, {}",
            in(reg) TCR_VALUE,
        );

        asm!(
            "msr ttbr0_el1, {}",
            in(reg) page_tables_start,
        );

        asm!("isb");
        asm!("tlbi vmalle1");
        asm!("ic iallu");
        asm!("dsb sy");
        asm!("isb");

        let mut sctlr: u64;
        asm!("mrs {}, sctlr_el1", out(reg) sctlr);
        sctlr |= 1 << 0;
        sctlr |= 1 << 2;
        sctlr |= 1 << 12;
        asm!("msr sctlr_el1, {}", in(reg) sctlr);

        asm!("isb");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sync_handler() {
    let esr: u64;  // Exception Syndrome Register
    let elr: u64;  // Exception Link Register (where it crashed)
    let far: u64;  // Fault Address Register

    unsafe {
        core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
        core::arch::asm!("mrs {}, elr_el1", out(reg) elr);
        core::arch::asm!("mrs {}, far_el1", out(reg) far);
    }

    if let Some(mut device) = critical_section::with(|cs| DEVICE.borrow(cs).get()) {
        let uart = &mut device.uart;
        writeln!(uart, "\nSYNC EXCEPTION:").ok();
        writeln!(uart, "ESR_EL1: 0x{:016x}", esr).ok();
        writeln!(uart, "ELR_EL1: 0x{:016x} (instruction address)", elr).ok();
        writeln!(uart, "FAR_EL1: 0x{:016x} (fault address)", far).ok();
        writeln!(uart, "EC (exception class): 0x{:x}", (esr >> 26) & 0x3F).ok();
    }

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
