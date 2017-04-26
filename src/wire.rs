use teensy3::bindings;


use bindings::Wire;

/**
  Initialize the two wire interface as a master
*/
pub fn begin_master()
{
    unsafe {
        Wire.begin();
    }
}

/**
  Close the two wire interface
*/
pub fn end()
{
    unsafe {
        Wire.end();
    }
}

/**
  Start sending a message to `address`
*/
pub fn begin_transmission(address: u8)
{
    unsafe {
        Wire.beginTransmission(address);
    }
}


/**
  Sends a single byte to the i2c device
*/
pub fn send_byte(byte: u8)
{
    unsafe {
        Wire.send(byte);
    }
}


/**
  Finnish sending the current message
*/
pub fn end_transmission()
{
    unsafe {
        Wire.endTransmission1();
    }
}


/**
  Read `quantity` bits from the device with `address`. The result is placed in `buffer`.

  The bytes read start at the memory pointer that can be set by writing a byte to the device

  Panics if `quantity` is less than buffer.len()
*/
pub fn request_from(address: u8, quantity: u8, buffer: &mut [u8]) -> u8
{
    unsafe {
        let received_amount = Wire.requestFrom1(address, quantity);

        assert!((received_amount as usize) < buffer.len());

        for i in 0..received_amount
        {
            buffer[i as usize] = Wire.receive();
        }

        received_amount
    }
}
