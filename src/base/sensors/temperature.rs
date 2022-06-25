use arduino_nano33iot::hal::adc::Adc;
use crate::base::prelude::*;

macro_rules! from {
    ($stone:expr, $pin: ident) => {{
        let pin = analog_pins::get_input!($stone, $pin);
        sensors::TemperatureSensor::new(pin)
    }};
}
pub(crate) use from;

pub struct TemperatureSensor<C> {
    pub pin: C
}

impl<C> TemperatureSensor<C> {
    pub fn new(pin: AnalogPin<C>) -> TemperatureSensor<AnalogPin<C>> {
        TemperatureSensor{pin: pin}
    }
}

impl<C> TemperatureSensor<C> where C: AnalogInput {
    pub fn get_temperature(&mut self, adc: &mut Adc<atsamd21g::ADC>) -> Result<f32, PinError> {
        return match self.pin.read(adc) {
            Ok(val) => Ok(val as f32),
            Err(_) => Err(PinError::ReadError)
        }
    }
}