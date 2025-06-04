/// This file handles the pins, the brightness level of
/// each RGB LED pin, and the timer for each pin.
/// The pulse width modulation is incorporated here as well
/// for altering the brightness of each individual color.
/// The methods ensure that the voltage of the knob is always
/// between 0 and 3.27 volts, allowing the brightness of a LED
/// pin to be either fully off (0), or fully on at its
/// brightest (15) as displayed in the UI.
use crate::*;

// An alias to the GPIO board pins that control LED:
type RgbPins = [Output<'static, AnyPin>; 3];

// Struct for lighting up each individual RGB position on the LED:
pub struct Rgb {
    rgb: RgbPins, // The pins for Red, Green, and Blue.
    // Shadow variables to minimize lock contention.
    levels: [u32; 3], // The level of brightness of a pin.
                      //tick_time: u64, // micro second ticker.
}

// All methods for the RGB struct.
impl Rgb {
    /// Micro seconds ticker for all 3 pins at the same
    /// time for a brightness level step.
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    /// To create a new Rgb object:
    pub fn new(rgb: RgbPins) -> Self {
        Self {
            // Return a new instance with the pins, the level, and total time.
            rgb,
            levels: [0; 3],
        }
    }

    async fn tick_time(level: u64) -> u64 {
        let frame_rate = get_frame_rate().await;
        let tick_time = Rgb::frame_tick_time(frame_rate);
        level * tick_time
    }

    /// PWM (pulse width modulation) for each color.
    async fn step(&mut self, led: usize) {
        let level = self.levels[led]; // The brightness of one of the RGB colors.

        if level > 0 {
            // Turn on specific LED pin a fraction of the frame time:

            self.rgb[led].set_high(); // ON

            //let on_time = level as u64 * self.tick_time; // Start the timer.
            // Change to be the shared async mutex:
            let on_time = Self::tick_time(level as u64).await;

            Timer::after_micros(on_time).await; // Async method for timing.
            self.rgb[led].set_low(); // OFF
        }

        // Find remainder of the ticker and sub in the brightness of the other
        // colors.
        let level = LEVELS - level;
        if level > 0 {
            //let off_time = level as u64 * self.tick_time;
            let off_time = Self::tick_time(level as u64).await;

            Timer::after_micros(off_time).await;
        }
    }

    /// Method to continuously run the RGB pulse width modulator.
    /// Loop through all the levels and step each color on the
    /// LED, altering the brightness for color changes while knob
    /// changes voltage to change the brightness.
    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;

            for led in 0..3 {
                // Go through each color in order of RGB.
                self.step(led).await;
            }
        }
    }
}
