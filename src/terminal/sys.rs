//! This module provides platform related functions.

#[cfg(any(unix, target_os = "wasi"))]
#[cfg(feature = "events")]
pub use self::unix::supports_keyboard_enhancement;
#[cfg(any(unix, target_os = "wasi"))]
pub(crate) use self::unix::{
    disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, size, window_size,
};
#[cfg(windows)]
#[cfg(feature = "events")]
pub use self::windows::supports_keyboard_enhancement;
#[cfg(all(windows, test))]
pub(crate) use self::windows::temp_screen_buffer;
#[cfg(windows)]
pub(crate) use self::windows::{
    clear, disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, scroll_down, scroll_up,
    set_size, set_window_title, size, window_size,
};

#[cfg(windows)]
mod windows;

#[cfg(any(unix, target_os = "wasi"))]
pub mod file_descriptor;
#[cfg(any(unix, target_os = "wasi"))]
mod unix;
