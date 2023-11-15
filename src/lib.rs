#[cfg_attr(target_os = "windows", path = "sleep_windows.rs")]
#[cfg_attr(not(target_os = "windows"), path = "sleep_default.rs")]
mod sleep;

use core::time::Duration;

use sleep::SleeperImpl;

pub struct Sleeper(SleeperImpl);

impl Sleeper {
    pub fn new() -> Self {
        Self(SleeperImpl::new())
    }

    /// # Safety
    /// Caller must ensure that there is only one thread that calls this method in a time.
    /// # Panics
    /// On low probability errors from Windows API.
    #[cfg_attr(not(target_os = "windows"), inline)]
    pub unsafe fn sleep(&mut self, duration: Duration) {
        unsafe {
            // SAFETY: This method have same safety requrements as caller method.
            self.0.sleep(duration);
        }
    }
}

impl Default for Sleeper {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Sleeper {
    /// Some APIs require resource cleanup
    /// So we add manual Drop here to make it dropped in every platform.
    /// It can prevent some multiplatform breakages.
    #[inline]
    fn drop(&mut self) {}
}

/// Calls platform specific precise sleeping routine for specified platform.
/// Note that it is just a convenient (and safe) wrapper over creating [`Sleeper`](crate::Sleeper) and calling
/// [`Sleeper::sleep`](crate::Sleeper::sleep) so it may be more effecient to create `Sleeper` once and keep it.
/// However, cost of creating `Sleeper`s are not very high so you can safely ignore it.
#[cfg_attr(not(target_os = "windows"), inline)]
pub fn sleep(duration: Duration) {
    unsafe {
        // SAFETY: We don't pass created timer to other threads.
        Sleeper::new().sleep(duration);
    }
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
        allowed_max: Duration,
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
        let maximum = *observations.iter().max().unwrap();

        assert!(
            mean < allowed_mean_fluctuation,
            "Mean exceeds allowed fluctuation {:?}>{:?}",
            mean,
            allowed_mean_fluctuation
        );

        assert!(
            maximum < allowed_max,
            "Max running time ({:?}) exceeds limit {:?}",
            maximum,
            allowed_max
        )
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
        assert_mean(
            &observations,
            duration,
            Duration::from_millis(1),
            duration + Duration::from_millis(2),
        );
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
            unsafe {
                // SAFETY: Current threads owns timer.
                sleeper.sleep(duration0);
            }
            let elapsed = start.elapsed();
            observations0.push(elapsed);

            let start = Instant::now();
            unsafe {
                // SAFETY: Current threads owns timer.
                sleeper.sleep(duration1);
            }
            let elapsed = start.elapsed();
            observations1.push(elapsed);
        }
        assert_mean(
            &observations0,
            duration0,
            Duration::from_millis(1),
            duration0 + Duration::from_millis(2),
        );
        assert_mean(
            &observations1,
            duration1,
            Duration::from_millis(1),
            duration1 + Duration::from_millis(2),
        );
    }
}
