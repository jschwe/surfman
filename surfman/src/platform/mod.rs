// surfman/surfman/src/platform/mod.rs
//
//! Platform-specific backends.

pub mod generic;

#[cfg(android)]
pub mod android;
#[cfg(android)]
pub use android as default;

#[cfg(ohos)]
pub mod ohos;
#[cfg(ohos)]
pub use ohos as default;

#[cfg(macos)]
pub mod macos;
#[cfg(macos)]
pub use macos::cgl as default;
#[cfg(macos)]
pub use macos::system;

#[cfg(all(linux, not(ohos)))]
pub mod unix;
#[cfg(all(linux, not(ohos)))]
pub use unix::default;

#[cfg(windows)]
pub mod windows;
#[cfg(angle_default)]
pub use windows::angle as default;
#[cfg(all(windows, not(angle_default)))]
pub use windows::wgl as default;
