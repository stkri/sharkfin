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

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("asm/aarch64/boot.aarch64.s"));
/// Main privileged space runtime of the kernel.\
/// **Do not edit unless you know what you are doing.**
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut u = unsafe { UART::new(115200) };
    let mut buf = [0x0; 64];
    writeln!(u, "SHARKFIN 0.1.0").unwrap();
    writeln!(u, "[ OK ]\tInitialized UART").unwrap();
    loop {
        write!(u, "[dsh] > ").unwrap();
        let mut pos: usize = 0;
        'get_cmd: loop {
            let c = u.read_byte().unwrap();
            match c {
                b'\0' => {},
                b'\n' | b'\r' => {
                    write!(u, "{}", c as char).unwrap();
                    break 'get_cmd;
                },
                b'\x08' | b'\x7F' => {
                    if pos == 0 {
                        write!(u, "\x07").unwrap();
                    } else {
                        write!(u, "\x7F\x08").unwrap();
                        pos -= 1;
                    }
                }
                c if pos < buf.len() => {
                    write!(u, "{}", c as char).unwrap();
                    buf[pos] = c;
                    pos += 1;
                }
                _ => write!(u, "\x07").unwrap(),
            }
        }
    }
}
/// Kernel panic handler.\
/// **Do not edit unless you know what you are doing.**
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut u = unsafe { UART::new(115200) };
    writeln!(u, "[KERNEL PANIC]").ok();
    writeln!(u, "{}", info).ok();
    loop {}
}
