#![no_std]
#![no_main]

use panic_halt as _;
use arduino_nano33iot as bsp;
use bsp::entry;

#[entry]
fn main() -> ! {
    loop {}
}
