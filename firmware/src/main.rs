#![no_std]
#![no_main]

#![allow(unused_imports)]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32l0::stm32l0x1::{interrupt, Interrupt};

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let _ret = common::do_something();
    
    loop {
        // your code goes here
    }
}