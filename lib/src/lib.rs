#![no_std]

//! # WS2812B
//!
//! A library to drive the WS2812B LED.
//!
//! # Play one LED
//! ```rust
//! ...
//!
//! let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
//! let peripherals: esp_hal::peripherals::Peripherals = esp_hal::init(config);
//!
//! let mut r = WS2812B::new(peripherals.RMT, 80, peripherals.GPIO8)?;
//!
//!  r = r.play(1)?;
//!
//! ...
//! ```
//!
//! # Build for different esp32 targets
//!
//! ```BASH
//! cargo run --features esp32c6
//! ```

pub mod ws2812_b;
pub use self::ws2812_b::WS2812B;
