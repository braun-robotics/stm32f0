#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
use cortex_m::asm;

extern crate stm32f0_hal;
use stm32f0_hal::gpio;

fn main() {
    let button = gpio::Input::setup(gpio::Pin::PA1);
    let mut led = gpio::Output::setup(gpio::Pin::PA5);

    loop {
        if button.read() {
            led.high();
        } else {
            led.low();
        }
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
