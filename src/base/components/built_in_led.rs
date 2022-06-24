use arduino_nano33iot::hal;
use hal::prelude::*;
use hal::gpio::v2::{PA17, Pin, PushPullOutput};

macro_rules! from {
    ($stone:expr) => {{
        use base::components::built_in_led::BuiltInLED;

        BuiltInLED::new($stone.pins.led_sck.into_push_pull_output())
    }};
}
pub(crate) use from;

pub struct BuiltInLED {
    pub pin: Pin<PA17, PushPullOutput>,
}

impl BuiltInLED {
    pub fn new(pin: Pin<PA17, PushPullOutput>) -> Self {
        BuiltInLED{pin: pin}
    }

    pub fn toggle(&mut self) {
        self.pin.toggle().unwrap();
    }

    pub fn set_high(&mut self) {
        self.pin.set_high().unwrap();
    }

    pub fn set_low(&mut self) {
        self.pin.set_low().unwrap();
    }
}