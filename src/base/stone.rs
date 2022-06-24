use arduino_nano33iot as bsp;

use atsamd21g::{NVMCTRL, PM, SYSCTRL, TC5};
use bsp::{hal, Pins};
use hal::clock::GenericClockController;
use hal::pac::Peripherals;

pub struct Stone {
    /// Power manager
    pub (crate) pm: PM,
    // Timer Counter 5
    pub (crate) tc5: TC5,
    /// System control
    pub (crate) sysctrl: SYSCTRL,
    /// Non-volitile memory controller
    pub (crate) nvmctrl: NVMCTRL,
    /// Clock
    pub (crate) clock: GenericClockController,
    /// Pins
    pub (crate) pins: Pins,
}

impl Stone {
    pub fn instance() -> Stone {
        let mut peripherals = Peripherals::take().unwrap();

        let clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL);

        let pins = bsp::Pins::new(peripherals.PORT);

        Stone {
            pm: peripherals.PM,
            tc5: peripherals.TC5,
            sysctrl: peripherals.SYSCTRL,
            nvmctrl: peripherals.NVMCTRL,
            clock: clocks,
            pins: pins
        }
    }
}