use std::time::Duration;

pub(crate) struct SleeperImpl;

impl SleeperImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn sleep(&mut self, duration: Duration) {
        std::thread::sleep(duration);
    }
}
