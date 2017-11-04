// copied from: https://github.com/robotrs/rust-wpilib/blob/f9d59c4f3ada64837c0e53ba7e9d85b00df882f5/src/wpilib/hal_call.rs
use std::{ffi, fmt};

pub mod bindings { 
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/hal-bindings.rs"));
}

use self::bindings::*;

/// Must be called first
pub fn init() -> HalResult<()> {
    if unsafe { HAL_Initialize(0) == 1 } {
        Ok(())
    } else {
        Err(HalError(0))
    }
}

#[derive(Copy, Clone)]
pub struct HalError(pub i32);

impl From<i32> for HalError {
    fn from(code: i32) -> Self {
        HalError(code)
    }
}

impl fmt::Debug for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let error_string = unsafe { ffi::CStr::from_ptr(HAL_GetErrorMessage(self.0)) };
        write!(f, "HalError {{ {} }}", error_string.to_str().unwrap())
    }
}

pub type HalResult<T> = Result<T, HalError>;

#[macro_export]
macro_rules! hal_call {
    ($function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(HalError::from(status)) }
    }};
    ($namespace:path, $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $namespace::$function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(HalError::from(status)) }
    }};
}