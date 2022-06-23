#![no_std]
#![no_main]

use panic_halt as _;
use arduino_nano33iot as bsp;
use bsp::entry;
use bsp::hal;
use hal::clock::GenericClockController;
use hal::pac::Peripherals;
use hal::prelude::*;

mod time;


#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL);
    let gclk0 = clocks.gclk0();
    let mut timer = time::Timer::new(peripherals.TC5, &mut clocks, &gclk0, &mut peripherals.PM);
    let mut led_last_toggled = timer.millis();

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led: bsp::Led = pins.led_sck.into();
    led.set_high().unwrap();

    loop {
        timer.tick();
        let now = timer.millis();
        if 500 < (now - led_last_toggled) {
            led.toggle().unwrap();
            led_last_toggled = now;
        }
    }
}
