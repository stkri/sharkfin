pub use crate::clock_addresses::CLOCK_COUNT_HIGH;
pub use crate::clock_addresses::CLOCK_COUNT_LOW;
pub use crate::clock_types::SystemClock;
pub use driver_traits::sys_clock::SystemClock as SysClockTrait;
use kernel_utils::nops::nop;

impl SysClockTrait for SystemClock {
    fn get_time(&self) -> u64 {
        let high = unsafe { CLOCK_COUNT_HIGH.read_volatile() } as u64;
        let low = unsafe { CLOCK_COUNT_LOW.read_volatile() } as u64;
        (high << 32) | low
    }

    fn sleep(&self, duration: core::time::Duration) {
        let time = self.get_time() + duration.as_micros() as u64;
        while self.get_time() < time {
            nop();
        }
    }
}
