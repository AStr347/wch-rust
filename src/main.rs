#![no_std]
#![no_main]

extern crate panic_halt;

use ch32v1::ch32v103::{Peripherals};
use riscv_rt::entry;

static mut STAT: u32 = 0;
static mut CFGO: u32 = 0;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    let dp = unsafe { Peripherals::steal() };
    let gpioa = &dp.GPIOA;
    gpioa
        .cfghr
        .write(|w| w.cnf10().variant(1).cnf12().variant(1));
    // do something here
    let mut i: bool = false;
    unsafe {
        STAT = gpioa.outdr.read().bits();
        CFGO = gpioa.cfghr.read().bits();
    }
    loop {
        if i {
            gpioa
                .outdr
                .write(|w| w.odr10().set_bit().odr12().clear_bit())
        } else {
            gpioa
                .outdr
                .write(|w| w.odr10().clear_bit().odr12().set_bit())
        }
        i = !i;
        unsafe {
            STAT = gpioa.outdr.read().bits();
            riscv::asm::delay(500);
        }
    }
}
