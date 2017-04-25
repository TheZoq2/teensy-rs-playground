build:
	@xargo build --release --target thumbv7em-none-eabihf

flash: build
	arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabihf/release/teensy3-rs-demo target/hex
	teensy-loader-cli -w -s --mcu=mk64fx512 target/hex

clean:
	xargo clean
	git clean -Xf
