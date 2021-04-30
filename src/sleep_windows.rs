use std::convert::TryInto;
use std::time::Duration;

use crate::win_bindings::{
    Windows::Win32::SystemServices::{
        CreateWaitableTimerExW, SetWaitableTimerEx, WaitForSingleObject,
        CREATE_WAITABLE_TIMER_HIGH_RESOLUTION, HANDLE, PWSTR, WAIT_RETURN_CAUSE,
    },
    Windows::Win32::WindowsProgramming::CloseHandle,
};
use windows::HRESULT;

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
            const TIMER_ALL_ACCESS: u32 = 0x1f0003;
            let handle = CreateWaitableTimerExW(
                std::ptr::null_mut(),
                PWSTR::NULL,
                CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
                TIMER_ALL_ACCESS,
            );
            if handle.is_null() {
                panic_on_win32_error("Failed to create waitable timer for punctual_timer")
            }
            handle
        };
        Self { timer }
    }

    pub(crate) fn sleep(&mut self, duration: Duration) {
        // Minimal resolution is 100 ns.
        const RESOLUTION_NS: i64 = 100;
        const CHUNK_PER_SEC: i64 = 10_000_000;
        let secs: i64 = duration
            .as_secs()
            .try_into()
            .expect("Too large sleep duration");
        let nanos: i64 = duration.subsec_nanos().into();
        // Use -1 to indicate relative time
        let sleep_time: i64 = -(nanos / RESOLUTION_NS + secs * CHUNK_PER_SEC);
        unsafe {
            SetWaitableTimerEx(
                self.timer,
                &sleep_time as *const i64,
                0,
                None,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
            )
            .ok()
            .expect("Failed to set waitable timer for punctual_timer");

            let wait_result = WaitForSingleObject(self.timer, u32::MAX);
            if wait_result == WAIT_RETURN_CAUSE::WAIT_FAILED {
                panic_on_win32_error("Failed to wait for timer in punctual_timer");
            }
        }
    }
}

// Safety: Should be called when some WinAPI operation failed
#[cold]
unsafe fn panic_on_win32_error(message: &'static str) -> ! {
    HRESULT::from_thread().ok().expect(message);
    unreachable!("Should have error because failed")
}
