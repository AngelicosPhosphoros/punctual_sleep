#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
fn main() {
    windows::build!(
        Windows::Win32::WindowsProgramming::CloseHandle,
        Windows::Win32::SystemServices::{
            WaitForSingleObject,CreateWaitableTimerExW, SetWaitableTimerEx,
            CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
        },
    );
}
