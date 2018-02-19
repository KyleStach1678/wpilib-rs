use wpilib::hal::*;
use wpilib::hal::bindings::*;
use wpilib::sensors;

/// A digital input used to read boolean sensors from the RoboRIO.
pub struct DigitalInput(HAL_DigitalHandle);

impl DigitalInput {
    pub fn new(channel: i32) -> HalResult<DigitalInput> {
        if !sensors::check_digital_channel(channel) {
            return Err(HalError(0));
        }

        let handle = hal_call!(HAL_InitializeDIOPort(HAL_GetPort(channel), true as i32))?;

        Ok(DigitalInput(handle))
    }

    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetDIO(self.0))? != 0)
    }

    pub fn get_handle(&self) -> HAL_Handle {
        self.0
    }
}

impl Drop for DigitalInput {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeDIOPort(self.0);
        }
    }
}
