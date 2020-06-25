//! OS-specific functionality.
//!
//! This corresponds to [`async_std::os`].
//!
//! [`async_std::os`]: https://docs.rs/async-std/latest/async_std/os/index.html

#[cfg(unix)]
pub mod unix;