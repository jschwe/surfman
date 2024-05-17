// surfman/surfman/src/platform/egl/mod.rs
//
//! Bindings to EGL on Android.

pub mod connection;
pub mod context;
pub mod device;
pub mod surface;

#[cfg(android_platform)]
mod android_ffi;

#[cfg(ohos_platform)]
mod ohos_ffi;

#[path = "../../implementation/mod.rs"]
mod implementation;

#[cfg(feature = "sm-test")]
#[path = "../../tests.rs"]
pub mod tests;
