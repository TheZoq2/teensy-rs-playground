DEVICE_FILE=/dev/ttyACM0

build:
	@xargo build --release --target thumbv7em-none-eabihf

flash: build
	arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabihf/release/teensy3-rs-demo target/hex
	teensy-loader-cli -w -s --mcu=mk64fx512 target/hex

monitor:
	screen ${DEVICE_FILE} 115200

doc:
	xargo doc --target thumbv7em-none-eabihf

clean:
	xargo clean
	git clean -Xf
