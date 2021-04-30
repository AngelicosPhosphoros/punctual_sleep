#[cfg_attr(target_os = "windows", path = "sleep_windows.rs")]
#[cfg_attr(not(target_os = "windows"), path = "sleep_default.rs")]
mod sleep;

#[cfg(target_os = "windows")]
mod win_bindings {
    windows::include_bindings!();
}

use sleep::SleeperImpl;
use std::time::Duration;

pub struct Sleeper(SleeperImpl);

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
    use std::convert::TryFrom;
    use std::time::{Duration, Instant};

    fn assert_mean(
        observations: &[Duration],
        target: Duration,
        allowed_mean_fluctuation: Duration,
    ) {
        for &dur in observations.iter() {
            assert!(
                target <= dur,
                "Sleep less than requested {:?} < {:?}",
                dur,
                target
            );
        }
        let sum: Duration = observations.iter().map(|&x| x - target).sum();
        let mean = sum / u32::try_from(observations.len()).unwrap();
        assert!(
            mean < allowed_mean_fluctuation,
            "Mean exceeds allowed fluctuation {:?}>{:?}",
            mean,
            allowed_mean_fluctuation
        );
    }

    #[test]
    fn test_sleep() {
        let duration = Duration::from_millis(5);
        let mut observations = Vec::new();
        for _ in 0..100 {
            let start = Instant::now();
            sleep(duration);
            let elapsed = start.elapsed();
            observations.push(elapsed);
        }
        assert_mean(&observations, duration, Duration::from_millis(1));
    }

    #[test]
    fn test_sleeper() {
        let mut sleeper = Sleeper::new();
        let duration0 = Duration::from_millis(5);
        let duration1 = Duration::from_millis(10);
        let mut observations0 = Vec::new();
        let mut observations1 = Vec::new();
        for _ in 0..100 {
            let start = Instant::now();
            sleeper.sleep(duration0);
            let elapsed = start.elapsed();
            observations0.push(elapsed);

            let start = Instant::now();
            sleeper.sleep(duration1);
            let elapsed = start.elapsed();
            observations1.push(elapsed);
        }
        assert_mean(&observations0, duration0, Duration::from_millis(1));
        assert_mean(&observations1, duration1, Duration::from_millis(1));
    }
}
