#![cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"))]

pub use self::monitor::{MonitorId, get_available_monitors, get_primary_monitor};
pub use self::window::{Window, XWindow, PollEventsIterator, WaitEventsIterator, Context, WindowProxy};
pub use self::xdisplay::{XConnection, XNotSupported};

pub mod ffi;

mod events;
mod input;
mod monitor;
mod window;
mod xdisplay;
