#![no_std]
#![no_main]

use panic_halt as _;
use arduino_nano33iot as bsp;
use bsp::entry;
use bsp::hal;
use hal::clock::GenericClockController;
use hal::pac::Peripherals;
use hal::prelude::*;
use atsamd21g::*;

mod time;
use time::Timer;

struct Stone {
    /// Power manager
    PM: PM,
    /// System control
    SYSCTRL: SYSCTRL,
    /// Non-volitile memory controller
    NVMCTRL: NVMCTRL,
    /// Clock
    CLOCK: GenericClockController,
    /// Timer(s)
    TIMER: Timer,
    /// On device LED
    LED: bsp::Led,
}

impl Stone {
    fn new() -> Stone {
        let mut peripherals = Peripherals::take().unwrap();

        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL);

        let gclk0 = clocks.gclk0();
        let timer = Timer::new(peripherals.TC5, &mut clocks, &gclk0, &mut peripherals.PM);

        let pins = bsp::Pins::new(peripherals.PORT);
        let led = pins.led_sck.into();

        Stone {
            PM: peripherals.PM,
            SYSCTRL: peripherals.SYSCTRL,
            NVMCTRL: peripherals.NVMCTRL,
            CLOCK: clocks,
            TIMER: timer,
            LED: led,
        }
    }
}

#[entry]
fn main() -> ! {
    let mut stone = Stone::new();
    let mut led_last_toggled = stone.TIMER.millis();

    stone.LED.set_high().unwrap();

    loop {
        stone.TIMER.tick();
        let now = stone.TIMER.millis();
        if 500 < (now - led_last_toggled) {
            stone.LED.toggle().unwrap();
            led_last_toggled = now;
        }
    }
}
