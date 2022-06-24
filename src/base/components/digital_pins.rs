use arduino_nano33iot::hal;
use hal::prelude::*;
use hal::gpio::v2::{PA17, Pin, PushPullOutput};
use hal::gpio::v2::pin::*;
use hal::hal::digital::v2::ToggleableOutputPin;

macro_rules! get_output {
    ($stone:expr, $port:ident) => {{
        use base::components::digital_pins::DigitalPin;

        DigitalPin::$port($stone.pins.$port.into_push_pull_output())
    }};
}
pub(crate) use get_output;


pub struct DigitalPin<C> {
    pub pin: C,
}

pub trait DigitalOutputPin {
    fn toggle(&mut self) -> ();
    fn set_high(&mut self) -> ();
    fn set_low(&mut self) -> ();
}

macro_rules! is_digital_output {
    ($d_id:ident, $port:ident) => {
        impl DigitalPin<Pin<$port, PushPullOutput>> {
            pub fn $d_id(pin: Pin<$port, PushPullOutput>) -> Self {
                DigitalPin{pin: pin}
            }
        }
        impl DigitalOutputPin for DigitalPin<Pin<$port, PushPullOutput>> {
            fn toggle(&mut self) {
                self.pin.toggle().unwrap();
            }
            fn set_high(&mut self) {
                self.pin.set_high().unwrap();
            }
            fn set_low(&mut self) {
                self.pin.set_low().unwrap();
            }
        }
    };
}

is_digital_output!(d2, PB10);
is_digital_output!(d3, PB11);
is_digital_output!(d4, PA07);
is_digital_output!(d5, PA05);
is_digital_output!(d6, PA04);
is_digital_output!(d7, PA06);
is_digital_output!(d8, PA18);
is_digital_output!(d9, PA20);
is_digital_output!(d10, PA21);
is_digital_output!(d11, PA16);
is_digital_output!(d12, PA19);
is_digital_output!(d13, PA17);
is_digital_output!(d14, PA02);
is_digital_output!(d15, PB02);
is_digital_output!(d16, PA11);
is_digital_output!(d17, PA10);
is_digital_output!(d18, PB08);
is_digital_output!(d19, PB09);
is_digital_output!(d20, PA09);
is_digital_output!(d21, PA03);