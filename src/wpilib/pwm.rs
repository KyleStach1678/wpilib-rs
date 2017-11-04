use wpilib::hal::*;
use wpilib::hal::bindings::*;
use wpilib::sensors;

pub struct PWM {
    handle: HAL_DigitalHandle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum PeriodMultiplier {
    K1X = 1,
    K2X = 2,
    K4X = 4,
}

impl PWM {

    /// Allocate a PWM given a channel number.
    ///
    /// Checks channel value range and allocates the appropriate channel.
    /// The allocation is only done to help users ensure that they don't double
    /// assign channels.
    pub fn new(channel: i32) -> HalResult<PWM> {
        if !sensors::check_pwm_channel(channel) {
            return Err(HalError(0));
        }

        let handle = hal_call!(HAL_InitializePWMPort(HAL_GetPort(channel)))?;

        hal_call!(HAL_SetPWMDisabled(handle))?;
        hal_call!(HAL_SetPWMEliminateDeadband(handle, 0))?;

        // TODO: report usage

        Ok(PWM {
            handle,
        })
    }

    /// Eliminate the deadband on a speed controller. If `eliminate` is true, there will be no
    /// deadband in the motor curve.
    pub fn eliminate_deadband(&mut self, eliminate: bool) -> HalResult<()> {
        hal_call!(HAL_SetPWMEliminateDeadband(self.handle, eliminate as i32))
    }

    /// Set the parameters for PWM pulses. All values are in milliseconds.
    /// # Arguments
    /// - `max` - the maximumum pulse width
    /// - `deadband_max` - the high end of the deadband
    /// - `center` - the center
    /// - `deadband_min` - the low end of the deadband
    /// - `min` - the minimum pulse width
    pub fn set_config(&mut self, max: f64, deadband_max: f64, center: f64, deadband_min: f64, min: f64) -> HalResult<()> {
        hal_call!(HAL_SetPWMConfig(self.handle, max, deadband_max, center, deadband_min, min))
    }

    /// Set the parameters for PWM pulses according to hardware values. All values are in hardware
    /// units (usually 0-2000 for a single cycle)
    /// # Arguments
    /// - `max` - the maximumum pulse width
    /// - `deadband_max` - the high end of the deadband
    /// - `center` - the center
    /// - `deadband_min` - the low end of the deadband
    /// - `min` - the minimum pulse width
    pub fn set_config_raw(&mut self,
                          max: i32,
                          deadband_max: i32,
                          center: i32,
                          deadband_min: i32,
                          min: i32)
                          -> HalResult<()> {
        hal_call!(HAL_SetPWMConfigRaw(self.handle, max, deadband_max, center, deadband_min, min))
    }

    /// Send a position for a servo, between 0 and 1.
    pub fn set_servo_position(&mut self, position: f64) -> HalResult<()> {
        hal_call!(HAL_SetPWMPosition(self.handle, position))
    }

    /// Set the PWM value based on a speed between -1 and 1.
    /// This call will fail if the PWM has not been set up properly (i.e. `set_config[_raw]` has not
    /// been called).
    pub fn set_speed(&mut self, speed: f64) -> HalResult<()> {
        hal_call!(HAL_SetPWMSpeed(self.handle, speed))
    }

    /// Get the most recently set speed.
    pub fn get_speed(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPWMSpeed(self.handle))
    }

    /// Set the PWM value in hardware terms (usually 0-2000)
    pub fn set_raw(&mut self, value: u16) -> HalResult<()> {
        hal_call!(HAL_SetPWMRaw(self.handle, value as i32))
    }

    /// Get the previously-set PWM value in hardware terms (usually 0-2000)
    pub fn get_raw(&self) -> HalResult<u16> {
        Ok(hal_call!(HAL_GetPWMRaw(self.handle))? as u16)
    }

    /// Slow down the period of the PWM signal cycle by a multiplier.
    ///
    /// Useful when using older devices that can't use a fast signal.
    pub fn slow_period(&mut self, multiplier: PeriodMultiplier) -> HalResult<()> {
        match multiplier {
            PeriodMultiplier::K1X => hal_call!(HAL_SetPWMPeriodScale(self.handle, 0)),
            PeriodMultiplier::K2X => hal_call!(HAL_SetPWMPeriodScale(self.handle, 1)),
            PeriodMultiplier::K4X => hal_call!(HAL_SetPWMPeriodScale(self.handle, 3)),
        }
    }

    /// Disable this PWM output until the next `set` or equivalent is called.
    pub fn disable(&mut self) -> HalResult<()> {
        hal_call!(HAL_SetPWMDisabled(self.handle))
    }

    /// Latch the PWM to zero.
    pub fn set_zero_latch(&mut self) -> HalResult<()> {
        hal_call!(HAL_LatchPWMZero(self.handle))
    }
}

impl Drop for PWM {
    fn drop(&mut self) {
        let _ = hal_call!(HAL_SetPWMDisabled(self.handle));
        let _ = hal_call!(HAL_FreePWMPort(self.handle));
    }
}