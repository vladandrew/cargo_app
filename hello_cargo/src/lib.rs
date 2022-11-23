#![no_std]

//use hex_literal::hex;

#[no_mangle]
extern "C" fn tfoo() -> u32{
	61     
}

use libm::acos;
use itoa;

pub fn it_works() -> u32 {

    let mut buffer = itoa::Buffer::new();
    let printed = buffer.format(128u64);
    let x = acos(10.0);
    10
}

