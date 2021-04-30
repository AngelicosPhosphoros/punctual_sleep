#[cfg_attr(target_os = "windows", path = "sleep_windows.rs")]
#[cfg_attr(not(target_os = "windows"), path = "sleep_default.rs")]
mod sleep;

#[cfg(target_os = "windows")]
mod win_bindings {
    windows::include_bindings!();
}

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

impl Drop for Sleeper {
    /// Some APIs require resource cleanup
    /// So we add manual Drop here to make it dropped in every platform.
    /// It can prevent some multiplatform breakages.
    #[inline]
    fn drop(&mut self) {}
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
        let duration = Duration::from_micros(500);
        let start = Instant::now();
        sleep(duration);
        let elapsed = start.elapsed();
        assert!(elapsed >= duration, "Too short {:?}", elapsed);
        assert!(
            elapsed <= duration + Duration::from_millis(1),
            "Too long {:?}",
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
        assert!(elapsed >= duration, "Too short {:?}", elapsed);
        assert!(
            elapsed <= duration + Duration::from_millis(1),
            "Too long {:?}",
            elapsed
        );

        let duration = Duration::from_millis(30);
        let start = Instant::now();
        sleeper.sleep(duration);
        let elapsed = start.elapsed();
        assert!(elapsed >= duration, "Too short {:?}", elapsed);
        assert!(
            elapsed <= duration + Duration::from_millis(1),
            "Too long {:?}",
            elapsed
        );
    }
}
