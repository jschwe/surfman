// surfman/surfman/src/implementation/connection.rs
//
//! This is an included private module that automatically produces the implementation of the
//! `Connection` trait for a backend.

use super::super::connection::{Connection, NativeConnection};
use super::super::device::{Adapter, Device, NativeDevice};
use super::super::surface::NativeWidget;
use crate::connection::Connection as ConnectionInterface;
use crate::info::GLApi;
use crate::Error;

use euclid::default::Size2D;

use std::os::raw::c_void;

#[deny(unconditional_recursion)]
impl ConnectionInterface for Connection {
    type Adapter = Adapter;
    type Device = Device;
    type NativeConnection = NativeConnection;
    type NativeDevice = NativeDevice;
    type NativeWidget = NativeWidget;

    #[inline]
    fn new() -> Result<Connection, Error> {
        Connection::new()
    }

    #[inline]
    fn native_connection(&self) -> Self::NativeConnection {
        Connection::native_connection(self)
    }

    #[inline]
    fn gl_api(&self) -> GLApi {
        Connection::gl_api(self)
    }

    #[inline]
    fn create_adapter(&self) -> Result<Adapter, Error> {
        Connection::create_adapter(self)
    }

    #[inline]
    fn create_hardware_adapter(&self) -> Result<Adapter, Error> {
        Connection::create_hardware_adapter(self)
    }

    #[inline]
    fn create_low_power_adapter(&self) -> Result<Adapter, Error> {
        Connection::create_low_power_adapter(self)
    }

    #[inline]
    fn create_software_adapter(&self) -> Result<Adapter, Error> {
        Connection::create_software_adapter(self)
    }

    #[inline]
    fn create_device(&self, adapter: &Adapter) -> Result<Device, Error> {
        Connection::create_device(self, adapter)
    }

    #[inline]
    unsafe fn create_device_from_native_device(
        &self,
        native_device: Self::NativeDevice,
    ) -> Result<Device, Error> {
        Connection::create_device_from_native_device(self, native_device)
    }

    #[inline]
    #[cfg(feature = "sm-raw-window-handle-05")]
    fn from_raw_display_handle(raw_handle: rwh_05::RawDisplayHandle) -> Result<Connection, Error> {
        Connection::from_raw_display_handle(raw_handle)
    }

    #[inline]
    #[cfg(feature = "sm-raw-window-handle-06")]
    fn from_display_handle(handle: rwh_06::DisplayHandle) -> Result<Connection, Error> {
        Connection::from_display_handle(handle)
    }

    #[inline]
    unsafe fn create_native_widget_from_ptr(
        &self,
        raw: *mut c_void,
        size: Size2D<i32>,
    ) -> NativeWidget {
        Connection::create_native_widget_from_ptr(self, raw, size)
    }

    #[inline]
    #[cfg(feature = "sm-raw-window-handle-05")]
    fn create_native_widget_from_raw_window_handle(
        &self,
        window: rwh_05::RawWindowHandle,
        size: Size2D<i32>,
    ) -> Result<NativeWidget, Error> {
        Connection::create_native_widget_from_raw_window_handle(self, window, size)
    }

    #[inline]
    #[cfg(feature = "sm-raw-window-handle-06")]
    fn create_native_widget_from_window_handle(
        &self,
        window: rwh_06::WindowHandle,
        size: Size2D<i32>,
    ) -> Result<NativeWidget, Error> {
        Connection::create_native_widget_from_window_handle(self, window, size)
    }
}
