# Homework by: Nathan Metens

## Board Setup

![Board Configuration](/PHOTO.jpg)

## LED Color and Frame Rate Demonstration

For the initial measurements of frame rate and RGB values, I set all defaults to their maximum levels:

- **Brightness**: 15 (for red, green, and blue)
- **Frame Rate**: 160

In my [short 49 sec video demonstration on YouTube](https://youtube.com/shorts/Z9_jB4t4RLI?feature=share) (![Also found here](/VIDEO.mov), I showcased the full range of the spectrum:

1. **Starting Point**
   All colors were set to maximum brightness, producing a white light from the LED.

2. **Color Reduction**
   - Decreased the **blue** value to 0 → LED turned **yellow**
   - Reduced the **green** value to 0 → LED turned **red**
   - Lowered the **red** value to 0 → LED turned **off** (black)

3. **Color Restoration**
   - Increased the **blue** brightness using the knob → LED showed **blue**
   - Increased the **green** brightness → LED showed **green**

## Frame Rate Observations

- Frame rates **below 40** result in a flickering or static LED.
- Frame rates **above 50** produce a smooth and stable display.
- **White light** occurs when all RGB values are equal:
  - Level 15 for each color → **Brightest white**
  - Level 1 for each color → **Dimmest white**

## Development Process

To accomplish this project, I followed these steps:

1. **Initial Exploration**
   I began by running the original program from the Git repository to observe how the blue light behaved.

2. **Code Review**
   Spent two days analyzing the code:
   - Added comments throughout
   - Referenced documentation
   - Broke down how each component functioned

3. **Modifications Made**
   - Made `frame_rate` and `rgb_colors` asynchronous to allow simultaneous UI access
   - Added a `current_led` variable to track which color cathode was active
   - Implemented `if-else` logic for button interactions by color

4. **Testing and Cleanup**
   - Thoroughly tested the modified code
   - Refactored for clarity
   - Commented all additions

## Reflection

This assignment went faster than expected because I front-loaded the time investment. Spending time to fully understand the structure and behavior of the program made implementing and debugging much easier. I learned a lot about how the different components interact and how to approach modifying and extending existing code.

## Original README intact below:

## rgbcal: RGB LED calibration tool
Bart Massey 2024

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

**XXX This tool is *mostly* finished! Please wire your
hardware up (see below), finish it, comment it, and use it
to find good values. Then document those values in this
README.**

### Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

### Wiring

Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

* Pin 1 to Gnd
* Pin 2 to P2
* Pin 3 to +3.3V

### UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
jus always controls Blue. You should see the color change
from green to teal-blue as you turn the knob clockwise.)

* No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160.
* A button held: Change the blue level from off to on over
  16 steps.
* B button held: Change the green level from off to on over
  16 steps.
* A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scanout code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason.

I think the frame rate is probably set higher than it needs
to be right now: it can be tuned lower.
