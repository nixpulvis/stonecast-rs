#![no_std]
#![no_main]

use panic_halt as _;

mod base;
use base::{entry, Stone};
use base::components::{built_in_led, timer, digital_pins};
use digital_pins::DigitalOutputPin;

#[entry]
fn main() -> ! {
    let mut stone = Stone::instance();
    let mut timer = timer::from!(stone);
    let mut built_in_led = built_in_led::from!(stone);
    let mut ext_led = digital_pins::get_output!(stone, d10);
    built_in_led.set_high();

    let mut led_last_toggled = timer.millis();
    loop {
        timer.tick();
        let now = timer.millis();
        if 500 < (now - led_last_toggled) {
            built_in_led.toggle();
            ext_led.toggle();
            led_last_toggled = now;
        }
    }
}
