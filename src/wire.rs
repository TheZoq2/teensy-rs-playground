use teensy3::bindings;


use bindings::Wire1;

/**
  Initialize the two wire interface as a master
*/
pub fn begin_master()
{
    unsafe {
        bindings::Wire1.begin();
        //bindings::TwoWire_begin(&mut Wire1);
        //bindings::TwoWire_begin(&mut bindings::Wire1);
    }
}

/*
    /**
      Close the two wire interface
    */
    pub fn end()
    {
        unsafe {
            Wire1.end();
        }
    }

    /**
      Start sending a message to `address`
    */
    pub fn begin_transmission(address: u8)
    {
        unsafe {
            Wire1.beginTransmission(address);
        }
    }


    /**
      Sends a single byte to the i2c device
    */
    pub fn send_byte(byte: u8)
    {
        unsafe {
            Wire1.send(byte);
        }
    }


    /**
      Finnish sending the current message
    */
    pub fn end_transmission()
    {
        unsafe {
            Wire1.endTransmission1();
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
            let received_amount = Wire1.requestFrom1(address, quantity);

            assert!((received_amount as usize) < buffer.len());

            for i in 0..received_amount
            {
                buffer[i as usize] = Wire1.receive();
            }

            received_amount
        }
    }
*/
