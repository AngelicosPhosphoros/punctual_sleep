use core::convert::TryInto;
use core::time::Duration;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_FAILED};
use windows::Win32::System::Threading::{
    CreateWaitableTimerExW, SetWaitableTimerEx, WaitForSingleObject,
    CREATE_WAITABLE_TIMER_HIGH_RESOLUTION, INFINITE,
};

pub(crate) struct SleeperImpl {
    timer: HANDLE,
}

impl Drop for SleeperImpl {
    fn drop(&mut self) {
        unsafe {
            // Safety: Called only once in destructor.
            // Object must have valid timer, see [new]
            CloseHandle(self.timer)
                .ok()
                .expect("Failed to close punctual_sleep timer");
        }
    }
}

impl SleeperImpl {
    pub(crate) fn new() -> Self {
        let timer: HANDLE = unsafe {
            // TODO: Check also TIMER_MODIFY_STATE https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-setwaitabletimerex
            const TIMER_ALL_ACCESS: u32 = 0x1f0003;
            let handle = CreateWaitableTimerExW(
                // We don't need security guarantees because we don't expect sleep timer to be sent to other processes.
                None,
                // Note:
                PCWSTR::null(),
                CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
                TIMER_ALL_ACCESS,
            );
            // I honestly don't know what calling application can do in such case.
            // And main reason of errors is collision of names for timers
            // which we don't use anyway so it is unlikely to happen.
            handle.expect("Failed to create waitable timer for punctual_timer")
        };
        Self { timer }
    }

    /// # Safety
    /// Caller must ensure that there is only one thread that calls this method in a time.
    pub(crate) unsafe fn sleep(&mut self, duration: Duration) {
        // Minimal resolution is 100 ns.
        //
        const RESOLUTION_NS: i64 = 100;
        const CHUNK_PER_SEC: i64 = 10_000_000;
        let secs: i64 = duration
            .as_secs()
            .try_into()
            .expect("Too large sleep duration");
        let nanos: i64 = duration.subsec_nanos().into();
        // Use negative value to indicate relative time.
        let sleep_time: i64 = -(nanos / RESOLUTION_NS + secs * CHUNK_PER_SEC);
        unsafe {
            SetWaitableTimerEx(
                self.timer,
                &sleep_time as *const i64,
                0,
                None,
                None,
                None,
                0,
            )
            .ok()
            .expect("Failed to set waitable timer for punctual_timer");

            let wait_result = WaitForSingleObject(self.timer, INFINITE);
            if wait_result == WAIT_FAILED {
                // Should not happen because we only waiting for a timer.
                panic!("Failed to wait on timer!");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SleeperImpl;

    use std::time::{Duration, Instant};

    #[test]
    fn test_sleeper_very_precise() {
        const TRIES: u16 = 100;
        let mut sleeper = SleeperImpl::new();
        let duration = Duration::from_micros(500);
        let mut times = Vec::with_capacity(TRIES.into());

        for _ in 0..TRIES {
            let start = Instant::now();
            unsafe {
                // SAFETY: Current thread owns the timer.
                sleeper.sleep(duration);
            }
            let elapsed = start.elapsed();
            times.push(elapsed);
        }

        assert!(times.iter().all(|&x| x >= duration));

        let mean = times.iter().copied().sum::<Duration>() / TRIES.into();
        assert!(
            mean < duration + Duration::from_millis(1),
            "Mean too big {:?}",
            mean
        );

        let maximum_derivation = times.iter().map(|&x| x - duration).max().unwrap();
        assert!(
            maximum_derivation < Duration::from_millis(2),
            "Derivation is too big {:?}",
            maximum_derivation
        );
    }
}
