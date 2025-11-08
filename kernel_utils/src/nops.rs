//! Collection of no-op instructions.
use core::hint::spin_loop;

/// Tell the CPU to wait for the given amount of cycles.
pub fn wait_cycles(cycles: usize) {
    for _ in 0..cycles {
        spin_loop();
    }
}
