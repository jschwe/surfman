use cgl::*;
use std::mem;

use platform::NativeGLContextMethods;

#[cfg(feature="texture_surface")]
use layers::platform::surface::NativeGraphicsMetadata;

pub struct NativeGLContext {
    native_context: CGLContextObj,
    pixel_format: CGLPixelFormatObj,
}

impl NativeGLContext {
    // NOTE: this function doesn't destroy the associated the
    //   corresponding CGLPixelFormatObj.
    pub fn new(share_context: Option<NativeGLContext>,
               pixel_format: CGLPixelFormatObj)
        -> Result<NativeGLContext, &'static str> {

        let shared = match share_context {
            Some(ctx) => ctx.as_native_cgl_context(),
            None => 0 as CGLContextObj
        };

        let mut native = unsafe { mem::uninitialized() };

        unsafe {
            if CGLCreateContext(pixel_format, shared, &mut native) != 0 {
                return Err("CGLCreateContext");
            }
        }

        debug_assert!(native != 0 as CGLContextObj);

        Ok(NativeGLContext {
            native_context: native,
            pixel_format: pixel_format,
        })
    }

    pub fn as_native_cgl_context(&self) -> CGLContextObj {
        self.native_context
    }
}

impl Drop for NativeGLContext {
    fn drop(&mut self) {
        unsafe {
            if CGLDestroyPixelFormat(self.pixel_format) != 0 {
                debug!("CGLDestroyPixelformat errored");
            }
            if CGLDestroyContext(self.native_context) != 0 {
                debug!("CGLDestroyContext returned an error");
            }
        }
    }
}

impl NativeGLContextMethods for NativeGLContext {
    fn create_headless() -> Result<NativeGLContext, &'static str> {
        // We try first with accelerated support
        let mut attributes = [
            0
        ];

        let mut pixel_format : CGLPixelFormatObj = unsafe { mem::uninitialized() };
        let mut pix_count = 0;

        unsafe {
            if CGLChoosePixelFormat(attributes.as_mut_ptr(), &mut pixel_format, &mut pix_count) != 0 {
                return Err("CGLChoosePixelFormat");
            }

            if pix_count == 0 {
                return Err("No pixel formats available");
            }
        }

        NativeGLContext::new(None, pixel_format)
    }

    fn is_current(&self) -> bool {
        unsafe {
            CGLGetCurrentContext() == self.native_context
        }
    }

    fn make_current(&self) -> Result<(), &'static str> {
        unsafe {
            if !self.is_current() &&
                CGLSetCurrentContext(self.native_context) != 0 {
                    Err("CGLSetCurrentContext")
            } else {
                Ok(())
            }
        }
    }

    #[cfg(feature="texture_surface")]
    fn get_metadata(&self) -> NativeGraphicsMetadata {
        NativeGraphicsMetadata {
            pixel_format: self.pixel_format,
        }
    }
}
