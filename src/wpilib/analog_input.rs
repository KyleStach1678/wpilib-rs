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
        let mut status: i32 = 0;
        let port = unsafe {
            HAL_InitializeAnalogInputPort(port, &mut status)
        };

        match status {
            0 => {
                Ok(AnalogInput {
                    channel,
                    port,
                })
            },
            _ => Err(HalError(status)),
        }
    }

    pub fn get_value(&self) -> i32 {
        let mut status: i32 = 0;
        // TODO: check error status
        unsafe {
            HAL_GetAnalogValue(self.port, &mut status)
        }
    }
    pub fn get_voltage(&self) -> f64 {
        let mut status: i32 = 0;
        // TODO: check error status
        unsafe {
            HAL_GetAnalogVoltage(self.port, &mut status)
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
