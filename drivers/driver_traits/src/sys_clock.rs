pub trait SystemClock {
    fn get_time(&self) -> u64;
    fn sleep(&self, duration: core::time::Duration);
}
