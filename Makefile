DEVICE_FILE=/dev/ttyACM0
ARCHITECTURE=thumbv7em-none-eabihf

build:
	@xargo build --release --target ${ARCHITECTURE}

flash: build
	arm-none-eabi-objcopy -O ihex -R .eeprom target/${ARCHITECTURE}/release/teensy3-rs-demo target/hex
	teensy-loader-cli -w -s --mcu=mk64fx512 target/hex

monitor:
	screen ${DEVICE_FILE} 115200

# Generate documentation and copy the result to the doc folder which clean won't eat
doc:
	xargo doc --target thumbv7em-none-eabihf
	cp target/${ARCHITECTURE}/doc doc -r

# Open the documentation using firefox
odoc:
	firefox doc/teensy3_sys/index.html
clean:
	xargo clean
	git clean -Xf
