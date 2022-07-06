use crate::base::prelude::*;
use crate::base::components::digital_pins::DigitalPin;
use arduino_nano33iot::hal::delay::Delay;
use crate::base::components::digital_pins::*;
use crate::base::components::error::PinError;

macro_rules! from {
    ($stone:expr, $pin: ident) => {{
        let pin = digital_pins::push_pull_output!($stone, $pin);
        indicators::Led::new(pin)
    }};
}
pub(crate) use from;

pub struct Led<C> {
    pub pin: DigitalPin<C>
}

impl<C> Led<C> where C: ToggleableOutputPin + OutputPin {
    pub fn new(pin: DigitalPin<C>) -> Led<C> {
        Led { pin: pin }
    }

    pub fn blink(&mut self, delay: &mut Delay) -> Result<(), PinError> {
        delay.delay_ms(1000u16);
        let mut result = self.pin.toggle();

        if result.is_ok() {
            delay.delay_ms(1000u16);
            result = self.pin.toggle();
        }

        return result;
    }

    pub fn toggle(&mut self) -> Result<(), PinError> {
        return self.pin.toggle();
    }

    pub fn set_high(&mut self) -> Result<(), PinError> {
        return self.pin.set_high();
    }

    pub fn set_low(&mut self) -> Result<(), PinError> {
        return self.pin.set_low();
    }
}
