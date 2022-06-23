#![no_std]
#![no_main]

use arduino_nano33iot as bsp;
use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,   
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led: bsp::Led = pins.led_sck.into();
    let mut ext_led = pins.d10.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut i = 0;

    loop {
        i += 1;

        delay.delay_ms(500u16);
        if (i % 10 == 0) {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

        if (i % 2 == 0) {
            ext_led.set_high().unwrap();
        } else {
            ext_led.set_low().unwrap();
        }
    }
}