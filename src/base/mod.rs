#![allow(dead_code)]
use arduino_nano33iot as bsp;
pub use bsp::entry;

pub mod components;

mod stone;
pub use stone::Stone;