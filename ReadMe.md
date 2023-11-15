# Punctual Sleep
This crate exists to provide more precise sleeping routine compared to [`std::thread::sleep`].

Main motivation to implement this crate was usage in games for Windows operating systems. On Windows, `std::thread::sleep` was as precise as 15 ms or worse so this crate was intended to fix that.
It manages to be in range of 2 ms of requested time almost always.

However, standard library was fixed after that to use [exact same method][1] as this crate so there is no need to do that anymore.

This code remains as easy to look example of such sleeping routine.
Another use-case is if you don't want to recreate WinAPI timer every time you need.

## Licensing and contribution
Crate licensed [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE20) license on your option.  
Any contributor (author of added code) to library agrees to distribute of his code by both licenses by making contribution.

[`std::thread::sleep`]: https://doc.rust-lang.org/std/thread/fn.sleep.html
[1]: https://github.com/rust-lang/rust/blob/dd430bc8c22f57992ec1457a87437d14283fdd65/library/std/src/sys/windows/thread.rs#L91
