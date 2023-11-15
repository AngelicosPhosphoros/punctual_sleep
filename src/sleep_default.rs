use std::time::Duration;

pub(crate) struct SleeperImpl;

impl SleeperImpl {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {}
    }

    #[inline]
    pub(crate) fn sleep(&mut self, duration: Duration) {
        // This is OK only in Linux, I fear.
        // However, I don't have access to any other system
        // so it is as good as any other fallback, I guess.
        std::thread::sleep(duration);
    }
}
