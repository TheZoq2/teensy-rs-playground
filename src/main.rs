#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use teensy3::bindings;
use teensy3::serial::Serial;
use teensy3::util::{
    analog_read,
    pin_mode,
    digital_write,
    delay,
    PinMode
};

mod wire;

fn slow_math() -> f32
{
    let iterations = 10000000;
    let mut result = 0.;

    for i in 0..iterations
    {
        result = 2. / (i % 10) as f32;
    }

    result
}

/**
  Benchmarks floating point operations
 */
fn test_flops()
{
    let start = unsafe{bindings::micros()};
    let slow_result = slow_math();
    let time = unsafe{bindings::micros()} - start;

    //Set the address we want to read some bytes from

    let micros_per_op = time as f32 / 10000.;

    println!("slow math {} calculated in {}, micros_per_op {}", slow_result, time, micros_per_op);

    println!("Analog value: {}, float: {}", analog_read(0), analog_read(0) as f32 / 1024.);
}

#[no_mangle]
pub extern fn main() {
    // Blink Loop

    pin_mode(13, PinMode::Output);
    digital_write(13, true);
    let mut ser = Serial{};

    wire::begin_master();

    loop {
        // Show we are alive
        unsafe {
            alive();
        }

        //Test i2c communication by reading temperature from a bmp085
        let device_addr = 0b1110111;
        //Set the memory address we want to read from
        let target_memory = 0x2E;
        wire::begin_transmission(device_addr);
        wire::send_byte(target_memory);
        wire::end_transmission();

        //read the result (a 16 bit integer)
        let mut buffer = [0; 2];
        wire::request_from(device_addr, 2, &mut buffer);
        let result = ((buffer[0] as u16) << 8) + buffer[1] as u16;

        println!("temperature value: {}", result);

        // If the serial write fails, we will halt (no more alive blinks)
        hello(&ser).unwrap();

        // Don't spam the console
        delay(1000);
    }
}

/// Blink the light twice to know we're alive
pub unsafe fn alive() {
    for _ in 0..2 {
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(200);
        bindings::digitalWrite(13, bindings::HIGH as u8);
        bindings::delay(200);
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(200);
    }
}

/// Send a message over the USB Serial port
pub fn hello(ser: &Serial) -> Result<(),()> {
    let msg = "Hello Teensy Rusty World!\n\r";
    ser.write_bytes(msg.as_bytes())
}
