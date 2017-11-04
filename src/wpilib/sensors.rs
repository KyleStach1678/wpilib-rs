use wpilib::hal::bindings::*;

pub fn get_digital_channels_num() -> i32 {
    unsafe {
        HAL_GetNumDigitalChannels()
    }
}

pub fn get_analog_inputs_num() -> i32 {
    unsafe {
        HAL_GetNumAnalogInputs()
    }
}

pub fn get_solenoid_channels_num() -> i32 {
    unsafe {
        HAL_GetNumSolenoidChannels()
    }
}

pub fn get_solenoid_modules_num() -> i32 {
    unsafe {
        HAL_GetNumPCMModules()
    }
}

pub fn get_pwm_channles_num() -> i32 {
    unsafe {
        HAL_GetNumPWMChannels()
    }
}

pub fn get_relay_channels_num() -> i32 {
    unsafe {
        HAL_GetNumRelayHeaders()
    }
}

pub fn get_pdp_channels_num() -> i32 {
    unsafe {
        HAL_GetNumPDPChannels()
    }
}

pub fn check_solenoid_module(module_number: i32) -> bool {
    unsafe {
        HAL_CheckSolenoidModule(module_number) == 1
    }
}

pub fn check_digital_channel(channel: i32) -> bool {
    unsafe {
        HAL_CheckDIOChannel(channel) == 1
    }
}

pub fn check_relay_channel(channel: i32) -> bool {
    unsafe {
        HAL_CheckRelayChannel(channel) == 1
    }
}

pub fn check_pwm_channel(channel: i32) -> bool {
    unsafe {
        HAL_CheckPWMChannel(channel) == 1
    }
}

pub fn check_analog_input_channel(channel: i32) -> bool {
    unsafe {
        HAL_CheckAnalogInputChannel(channel) == 1
    }
}

pub fn check_analog_output_channel(channel: i32) -> bool {
    unsafe {
        HAL_CheckAnalogOutputChannel(channel) == 1
    }
}

pub fn check_solenoid_channel(channel: i32) -> bool {
    unsafe {
        HAL_CheckSolenoidChannel(channel) == 1
    }
}

pub fn check_pdp_channel(channel: i32) -> bool {
    unsafe {
        HAL_CheckPDPChannel(channel) == 1
    }
}

pub fn check_pdp_module(channel: i32) -> bool {
    unsafe {
        HAL_CheckPDPModule(channel) == 1
    }
}