#![no_std]
#![no_main]
#![allow(dead_code)]

// esp_hal docs:
// https://docs.espressif.com/projects/rust/esp-hal/1.0.0-beta.0/esp32c6/esp_hal/index.html

use esp_hal::{clock::CpuClock, main, riscv::asm::nop};
use esp_ws2812_b::WS2812B;

const GPIO_RGB: u32 = 8;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // println!(
    //     "Panic reason: {} location: {:?}",
    //     info.message(),
    //     info.location()
    // );
    loop {}
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals: esp_hal::peripherals::Peripherals = esp_hal::init(config);

    let mut r =
        WS2812B::new(_peripherals.RMT, 80, _peripherals.GPIO8).expect("Failed to create BRG!");
    r.set_colors(255, 0, 100);
    r = r.play(1).expect("Failed to dispatch!");

    loop {
        nop();
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
