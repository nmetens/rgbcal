#![no_std]
#![no_main]

// This is the main program that brings all the
// modules together.

mod knob;
mod rgb;
mod ui;

// Accessing each module in its entirety:
pub use knob::*;
pub use rgb::*;
pub use ui::*;

use panic_rtt_target as _; // Panic handler for Real Time Transfer.
use rtt_target::{rprintln, rtt_init_print}; // Grabbing the RTT methods.

// Use of embassy crates and imports:
use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer; // https://docs.rs/embassy-time/latest/embassy_time/

// Microbit crate and embedded HAL:
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    Button, Microbit,
};
use num_traits::float::FloatCore; // Floats for math operations.

// Protect the shared data between the levels and RGB colors:
pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);

// Create a shareable mutex for the fram_rate between the Ui and RGB structs.
pub static FRAME_RATE: Mutex<ThreadModeRawMutex, u64> = Mutex::new(50); // Default the frame rate: to 50 Hz.

pub const LEVELS: u32 = 16; // Constant to set the RGB levels.

// Create the Async method to share the frame_rate between the Ui and RGB structs:
async fn get_frame_rate() -> u64 {
    // Follow get_rgb_levels function.
    let frame_rate = FRAME_RATE.lock().await;
    *frame_rate
}

// Setting/changing the frame rate... Followed set_rgb_levels method:
async fn set_frame_rate<F>(setter: F)
where
    F: FnOnce(&mut u64),
{
    let mut frame_rate = FRAME_RATE.lock().await;
    setter(&mut frame_rate);
}

// Asynchronous method to modify the RGB levels:
async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

// Asynchronous function to change the RGB levels of brightness:
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

// Entry point for ansynchronous embedded program for RGB:
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    rtt_init_print!(); // Initialize logging.
    let board = Microbit::default(); // Capture the board in a variable.

    // Bind the Successive Approximation Analog-to-Digital Converter.
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    // Holds all the pins on the LED:
    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);

    // Location on the GPIO board where each RGB pin is connected.:
    let red = led_pin(AnyPin::from(board.p9)); // Red LED pin on GPIO pin P9.
    let green = led_pin(AnyPin::from(board.p8)); // Green LED pin connected to GPIO on P8.
    let blue = led_pin(AnyPin::from(board.p16)); // Blue LED pin connects to P16 on GPIO board.
    let rgb: Rgb = Rgb::new([red, green, blue]); // All GPIO pins in the RGB struct with 100 micro s tick time.

    // Config analog-to-digital converter for knob to read knob dial input:
    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;

    // Config the saadc to connect to P2 on the GPIO board:
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );

    // Initialize the Knob object asynchronously:
    let knob = Knob::new(saadc).await;

    // Create a mutable object of the user interface passing in
    // the knob object and the two buttons on the microbit:
    let mut ui = Ui::new(knob, board.btn_a, board.btn_b);

    // Run both the RGB loop and UI loop asynchronously:
    join::join(rgb.run(), ui.run()).await;

    // Error occured above in the join...
    panic!("fell off end of main loop");
}
