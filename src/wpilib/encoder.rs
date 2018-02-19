use wpilib::hal::*;
use wpilib::hal::bindings::*;
use wpilib::digital_input::DigitalInput;

pub struct Encoder {
    source_a: DigitalInput,
    source_b: DigitalInput,
    source_index: Option<DigitalInput>,
    handle: HAL_EncoderHandle,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum EncodingType {
    Single,
    Double,
    Quadruple,
}

impl From<EncodingType> for HAL_EncoderEncodingType {
    fn from(e: EncodingType) -> HAL_EncoderEncodingType {
        match e {
            EncodingType::Single => HAL_EncoderEncodingType::HAL_Encoder_k1X,
            EncodingType::Double => HAL_EncoderEncodingType::HAL_Encoder_k2X,
            EncodingType::Quadruple => HAL_EncoderEncodingType::HAL_Encoder_k4X,
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum IndexingType {
    ResetWhileHigh,
    ResetWhileLow,
    ResetOnFallingEdge,
    ResetOnRisingEdge,
}

impl From<IndexingType> for HAL_EncoderIndexingType {
    fn from(i: IndexingType) -> HAL_EncoderIndexingType {
        match i {
            IndexingType::ResetWhileHigh => HAL_EncoderIndexingType::HAL_kResetWhileHigh,
            IndexingType::ResetWhileLow => HAL_EncoderIndexingType::HAL_kResetWhileLow,
            IndexingType::ResetOnFallingEdge => HAL_EncoderIndexingType::HAL_kResetOnFallingEdge,
            IndexingType::ResetOnRisingEdge => HAL_EncoderIndexingType::HAL_kResetOnRisingEdge,
        }
    }
}

impl Encoder {
    pub fn new(channel_a: i32, channel_b: i32) -> HalResult<Encoder> {
        let source_a = DigitalInput::new(channel_a)?;
        let source_b = DigitalInput::new(channel_b)?;

        let handle = hal_call!(HAL_InitializeEncoder(source_a.get_handle(),
                                            HAL_AnalogTriggerType::HAL_Trigger_kInWindow,
                                            source_b.get_handle(),
                                            HAL_AnalogTriggerType::HAL_Trigger_kInWindow,
                                            false as i32,
                                            From::from(EncodingType::Single)))?;

        Ok(Encoder {
            source_a: source_a,
            source_b: source_b,
            source_index: None,
            handle: handle,
        })
    }

    /// Set the index to a specified DigitalSource. When the index pulse is triggered, the encoder
    /// will reset (based on the indexing type):
    ///  - ResetWhileHigh: whenever the index pulse is on, the encoder will be zero.
    ///  - ResetWhileLow: whenever the index pulse is off, the encoder will be zero.
    ///  - ResetOnFallingEdge: whenever there is a falling edge (on -> off), the encoder will
    ///  reset.
    ///  - ResetOnRisingEdge: whenever there is a rising edge (off -> on), the encoder will reset.
    pub fn set_index(&mut self, index_channel: i32, index_type: IndexingType) -> HalResult<()> {
        let source_index = DigitalInput::new(index_channel)?;
        hal_call!(HAL_SetEncoderIndexSource(self.handle,
                                            source_index.get_handle(),
                                            HAL_AnalogTriggerType::HAL_Trigger_kInWindow,
                                            From::from(index_type)))?;

        self.source_index = Some(source_index);
        Ok(())
    }

    /// Get the scaled value of the encoder
    pub fn get(&self) -> i32 {
        hal_call!(HAL_GetEncoder(self.handle)).unwrap()
    }

    /// Get the unscaled value of the encoder
    pub fn get_raw(&self) -> i32 {
        hal_call!(HAL_GetEncoderRaw(self.handle)).unwrap()
    }

    /// Get the encoding scale, either 1, 2, or 4
    pub fn get_encoding_scale(&self) -> i32 {
        hal_call!(HAL_GetEncoderEncodingScale(self.handle)).unwrap()
    }

    /// Reset the encoder's position to zero
    pub fn reset(&mut self) {
        hal_call!(HAL_ResetEncoder(self.handle)).unwrap()
    }

    /// The period of the most recent encoder pulse, in seconds
    pub fn get_period(&self) -> f64 {
        hal_call!(HAL_GetEncoderPeriod(self.handle)).unwrap()
    }

    /// Set the maximum period before it's considered "not moving"
    pub fn set_max_period(&mut self, max_period: f64) {
        hal_call!(HAL_SetEncoderMaxPeriod(self.handle, max_period)).unwrap();
    }

    /// Is the encoder stopped? In other words, is the latest pulse width (the time since the
    /// latest rising or falling edge) longer than the max_period that was set earlier?
    pub fn is_stopped(&self) -> bool {
        hal_call!(HAL_GetEncoderStopped(self.handle)).unwrap() != 0
    }

    /// Was the last edge a move forwards (true) or backwards (false)?
    pub fn is_direction_forward(&self) -> bool {
        hal_call!(HAL_GetEncoderDirection(self.handle)).unwrap() != 0
    }

    /// Set how many samples should be averaged when getting from the encoder
    pub fn set_samples_to_average(&mut self, num_samples: i32) -> HalResult<()> {
        hal_call!(HAL_SetEncoderSamplesToAverage(self.handle, num_samples))
    }

    /// Get a reference to the A channel
    pub fn channel_a<'a>(&'a self) -> &'a DigitalInput {
        &self.source_a
    }

    /// Get a reference to the B channel
    pub fn channel_b<'a>(&'a self) -> &'a DigitalInput {
        &self.source_b
    }

    /// Get a reference to the index pulse channel
    pub fn channel_index<'a>(&'a self) -> Option<&'a DigitalInput> {
        match self.source_index {
            Some(ref index) => Some(index),
            None => None
        }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        hal_call!(HAL_FreeEncoder(self.handle)).unwrap();
    }
}
