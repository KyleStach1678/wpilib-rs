use wpilib::hal::*;
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
        hal_call!(HAL_InitializeAnalogInputPort(port))?;

        Ok(AnalogInput {
            channel,
            port,
        })
    }

    /// Returns 0 on error
    pub fn get_value(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogValue(self.port))
    }
    /// Returns 0 on error
    pub fn get_voltage(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogVoltage(self.port))
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
