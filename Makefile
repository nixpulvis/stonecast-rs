PORT := /dev/tty.usbmodem144401

build:
	cargo build --release

flash: build
	rust-objcopy -O binary target/thumbv6m-none-eabi/release/stonecast target/arduino.bin
	arduino-cli upload -i target/arduino.bin -b arduino:samd:nano_33_iot -p ${PORT}
