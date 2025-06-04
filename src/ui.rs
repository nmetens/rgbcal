/// The user interface module that shows the user
/// the brightness level of each LED in the RGB pins
/// and the frame rate of the LED. The UI keeps track
/// of the knob state, and updates LED brightness
/// accordingly.
use crate::*;

/// Struct that holds the values (levels and frame_rate).
/// levels is the brightness level of one of the RGB colors
/// in the LED. frame_rate is the frame_rate of the LED
/// updates (it's the time to scan out all three colors).
struct UiState {
    levels: [u32; 3], // Brightness level for each RGB color.
    frame_rate: u64,  // Frame rate for pulse width modulation.
    // Since levels is indexed from 0 - 2, we start with red at 0:
    current_led: usize, // RGB: 0 -> Red, 1 -> Green, 2 -> Blue.
}

// Implementation of the show method for the UiState struct.
impl UiState {
    /// Method that displays the color, brightness from 0 - 15,
    /// and the frame rate at which the PWM occurs:
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();

        // Loop through RGB to display their levels.
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }

        // Display fr for each color in the RGB led:
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

/// Set the decault values for levels and frame_rate.
impl Default for UiState {
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1], // All colors at LEVELS - 1 (full brightness).
            frame_rate: 50,                               // Pulse Width rate.
            current_led: 2,                               // Default start with Blue.
        }
    }
}

/// User interface to interact with the knob, and the
/// buttons on the microbit to change each color.
pub struct Ui {
    knob: Knob, // The knob to determine the LEVELS brightness of a specific LED pin (red, gree, or blue).
    _button_a: Button, // A button held: Change the blue level from off to on over 16 steps.
    _button_b: Button, // B button held: Change the green level from off to on over 16 steps.
    state: UiState, // Keeps track of the current UiState struct object's values (levels and frame_rate).
}

// Implemenation methods for the User Interface struct:
impl Ui {
    /// Simple constructor for the Ui struct above that takes microbit buttons,
    /// the knob, and the UiState for default brightness and frame_rate of the LED.
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    async fn update_level(&mut self, current_color: usize) {
        let level = self.knob.measure().await;
        if level != self.state.levels[current_color] {
            self.state.levels[current_color] = level;
            set_rgb_levels(|rgb| {
                *rgb = self.state.levels;
            })
            .await;
            self.state.show();
        }
    }

    /// Main method for User Interface. In this particular case,
    /// since the levels[2] is used, we are only changing the Blue LED.
    /// [1] and [3] would be red and green respectively.
    /// This looping method takes in the knob value and voltage to change
    /// the brightness of the blue LED pin.
    pub async fn run(&mut self) -> ! {
        let current_color = self.state.current_led;
        self.state.levels[current_color] = self.knob.measure().await; // Read blue's knob level.

        let current_frame_rate: u64 = ((self.knob.measure().await + 1) * 10).into(); // 10 to 160 (assuming LEVELS = 15)
        self.state.frame_rate = current_frame_rate;

        set_rgb_levels(|rgb| {
            // Set rgb levels from the LED controls.
            *rgb = self.state.levels;
        })
        .await; // Asyncronous value.

        // Share initial frame_rate values exclusively:
        //set_frame_rate(self.state.frame_rate);
        set_frame_rate(|rate| *rate = self.state.frame_rate).await; // Async sharing between resources.

        self.state.show(); // Show the initial state for the blue brightness depending on the original knob position.

        // Loop and wait for knob changes to update the brightness of the blue LED pin:
        loop {
            // RGB: RED, GREEN, BLUE...
            // The longest lead on the LED is ground (cathode).
            // If you look at the LED so that ground is in position 2, the positions are:
            //     Red: 1
            //     Gnd: 2
            //     Green: 3
            //     Blue: 4
            // However, our levels start at 0. So, red == 0, green == 1, blue == 2...

            // Create button variables for simplicity
            let button_a_held = self._button_a.is_low();
            let button_b_held = self._button_b.is_low();

            if button_a_held && button_b_held {
                // A+B buttons held: Change the red level from off to on over 16 steps.
                // RED (0):
                // Check if both the a and the b buttons are pressed on the microbit board:
                self.state.current_led = 0; // Set the current led to red.
            } else if button_b_held {
                // B button held: Change the green level from off to on over 16 steps.
                // GREEN:
                // Check if the b button is pressed on the microbit board:
                self.state.current_led = 1; // Set the current led to green
            } else if button_a_held {
                // A button held: Change the BLUE level from off to on over 16 steps.
                // BLUE:
                // Check if the a button is pressed on the microbit board:
                self.state.current_led = 2; // Set the current led to blue.
            }

            if button_a_held || button_b_held {
                self.update_level(self.state.current_led).await; // Call the method to update the color brightness with the knob.
            } else {
                // Frame Rate:
                // If no buttons are held, then change the frame rate in steps of 10 frames per second from 10..160:
                // if !self._button_a.is_low() && !self._button_b.is_low() {
                let current_frame_rate: u64 = ((self.knob.measure().await + 1) * 10).into(); // 10 to 160 (assuming LEVELS = 15)
                if current_frame_rate != self.state.frame_rate {
                    self.state.frame_rate = current_frame_rate;
                    set_frame_rate(|rate| {
                        *rate = self.state.frame_rate;
                    })
                    .await;
                    self.state.show();
                }
            }
            Timer::after_millis(50).await; // CPU rest.
        }
    }
}
