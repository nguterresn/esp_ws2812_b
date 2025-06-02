#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::println;
use esp_ws2812_b::WS2812B;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Panic! {}, l: {:?}", info.message(), info.location());
    loop {}
}

#[main]
fn main() -> ! {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let mut r = WS2812B::new(_peripherals.RMT, 80, _peripherals.GPIO7).expect("Failed to init");
    r.set_white(50);
    r = r.play(1).expect("Failed to play");

    loop {
        r = r.fade(1).expect("Failed to play");

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
