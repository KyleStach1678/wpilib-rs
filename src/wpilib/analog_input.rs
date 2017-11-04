use wpilib::hal::bindings::*;

pub struct AnalogInput {
    channel: i32,
    port: HAL_AnalogInputHandle,
}

impl AnalogInput {
    /// Create an `AnalogInput` instance from a channel number
    /// Currently does not do bound-checking
    /// Returns `HalResult<AnalogInput>`
    pub fn new(channel: i32) -> HalResult<AnalogInput> {
        // todo: bounds checking on channel
        let port = unsafe { HAL_GetPort(channel) };
        let port = unsafe {
            HAL_InitializeAnalogInputPort(port, &mut status)
        };
        match hal_call!(HAL_InitializeAnalogInputPort(port)) {
            Ok(port) => Ok(AnalogInput {
                channel,
                port,
            }),
            Err(e) => Err(e),
        }
    }

    /// Returns 0 on error
    pub fn get_value(&self) -> i32 {
        // TODO: check error status
        // for now, return 0 on error
        match hal_call!(HAL_GetAnalogValue(self.port)) {
            Ok(v) => v,
            Err(_) => 0,
        }
    }
    /// Returns 0 on error
    pub fn get_voltage(&self) -> f64 {
        // TODO: check error status
        // for now, return 0 on error
        match hal_call!(HAL_GetAnalogVoltage(self.port)) {
            Ok(v) => v,
            Err(_) => 0,
        }
    }
}

impl Drop for AnalogInput {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeAnalogInputPort(self.port);
        }
        self.port = HAL_kInvalidHandle as i32;
    }
}
