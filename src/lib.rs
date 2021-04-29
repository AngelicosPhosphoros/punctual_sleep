mod sleep_default;

use sleep_default as sleep;

use sleep::SleeperImpl;
use std::time::Duration;

struct Sleeper(SleeperImpl);

impl Sleeper {
    pub fn new() -> Self {
        Self(SleeperImpl::new())
    }

    pub fn sleep(&mut self, duration: Duration) {
        self.0.sleep(duration)
    }
}

pub fn sleep(duration: Duration) {
    Sleeper::new().sleep(duration);
}

#[cfg(test)]
mod tests {
    use super::{sleep, Sleeper};
    use std::time::{Duration, Instant};
    #[test]
    fn test_sleep() {
        let duration = Duration::from_millis(10);
        let start = Instant::now();
        sleep(duration);
        let elapsed = start.elapsed();
        assert!(elapsed >= duration, "{:?}", elapsed);
        assert!(
            elapsed <= duration + Duration::from_millis(1),
            "{:?}",
            elapsed
        );
    }

    #[test]
    fn test_sleeper() {
        let mut sleeper = Sleeper::new();

        let duration = Duration::from_millis(10);
        let start = Instant::now();
        sleeper.sleep(duration);
        let elapsed = start.elapsed();
        assert!(elapsed >= duration, "{:?}", elapsed);
        assert!(
            elapsed <= duration + Duration::from_millis(1),
            "{:?}",
            elapsed
        );

        let duration = Duration::from_millis(30);
        let start = Instant::now();
        sleeper.sleep(duration);
        let elapsed = start.elapsed();
        assert!(elapsed >= duration, "{:?}", elapsed);
        assert!(
            elapsed <= duration + Duration::from_millis(1),
            "{:?}",
            elapsed
        );
    }
}
