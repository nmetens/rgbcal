# RGB LED Calibration 
By Nathan Metens

## Board Setup

![Board Configuration](/PHOTO.jpg)

---

### LED Color and Frame Rate Demonstration

For the initial measurements of frame rate and RGB values, I set all defaults to their maximum levels:

- **Brightness**: 15 (for red, green, and blue)
- **Frame Rate**: 160

In my [short 49 sec video demonstration on YouTube](https://youtube.com/shorts/Z9_jB4t4RLI?feature=share) (![Also found here](/VIDEO.mov)), I showcased the full range of the spectrum:

1. **Starting Point**
   All colors were set to maximum brightness, producing a white light from the LED.

2. **Color Reduction**
   - Decreased the **blue** value to 0 → LED turned **yellow**
   - Reduced the **green** value to 0 → LED turned **red**
   - Lowered the **red** value to 0 → LED turned **off** (black)

3. **Color Restoration**
   - Increased the **blue** brightness using the knob → LED showed **blue**
   - Increased the **green** brightness → LED showed **green**

---

### Frame Rate (Refresh Rate) Flickering Observations

The refresh rate at which the LED appears stable (no perceptible flicker) is **at least 60 frames per second (fps)**:

- **50 fps**: Slight flicker is noticeable when compared to 60 fps.
- **40 fps**: Flicker is clearly visible.
- **30 fps**: Flicker is obvious and distracting.
- **≥ 60 fps**: Steady, continuous light is perceived. Higher frame rates do not provide noticeable improvement.

**Conclusion:**  
A minimum frame rate of **60 fps** is recommended for smooth color transitions and clear display of white. A maximum refresh rate of 160 fps is impractical, as it would overuse the CPU for a visual equivalent to 60 fps.

---

### White Light Appearance and Brightness Levels

To achieve a decent white color using the RGB LED:

- When **all color brightness levels** are at their maximum (**Red = Green = Blue = 15**), the light appears clearly white.
- When **brightness = 1** for each color, the LED output is not perceived as white; individual colors are easily distinguishable.
- At **brightness = 5**, the LED is bright, but the colors remain distinct.
- At **brightness = 10**, using a paper/tape diffuser, the colors begin to blend and form a white-ish light.
- At **brightness = 12**, with the diffuser on the LED, it appears **white** to my eye and to my peers.
- At **brightness = 15**, the white is at its strongest and most stable.

**Conclusion:**  
To achieve a perceptible white light, a brightness level of at least **12/16 (75%)** is recommended per color channel.

---

### Pulse Width Modulation (PWM) Timing and Calculations

The LED uses PWM to set brightness:

- **Total frame time**:

As mentioned before, there is no visual difference between 60 fps and 120 fps, so **60 fps** is the acceptable minimum.

Frame rate = 1 / 60 = 16.67 ms

The frame is divided across **three LED channels, Red, Green, and Blue (RGB)** -> each LED shares the time equally:

Slot time per color = 16.67 ms / 3 = 5.555 ms

The PWM operates with **16 levels (0–15)** -> **tick time**:

![frame time](/images/frame_tick_time.png)

Tick time = 1,000,000 µs / (3 x 60 x 16) = 347.22 µs

- **ON time for brightness level 12**:

Brightness level 12 was perceived to be the minimum level at which the color white was detectable with the difuser.

![on time](/images/tick_time.png)

![on time](/images/tick_time_res.png)

on_time = 12 x 347.22 µs = 4.17 ms

- **ON time for brightness level 13**:

The higher the brightness after level 12, the more white becomes apparent. With level 13 brightness, we see on time:

on_time = 13 x 347.22 µs = 4.51 ms

- **ON time for brightness level 14**:

on_time = 14 x 347.22 µs = 4.86 ms

- **ON time for brightness level 15**:

on_time = 15 x 347.22 µs = 5.21 ms

- **ON time for brightness level 16**:

on_time = 16 x 347.22 µs = 5.55 ms

- **Percentage of time the LED is ON**:

- For level 12 brightness, on_time percentage = (4.17 ms / 5.555 ms) x 100% ≈ 75%
- For level 13 brightness, on_time percentage = (4.51 ms / 5.555 ms) x 100% ≈ 81%
- For level 14 brightness, on_time percentage = (4.86 ms / 5.555 ms) x 100% ≈ 87%
- For level 15 brightness, on_time percentage = (5.21 ms / 5.555 ms) x 100% ≈ 94%
- For level 16 brightness, on_time percentage = (5.55 ms / 5.555 ms) x 100% ≈ 100%

---

### Summary & Answer to Question

**Question:** Make the measurements and provide an approximate minimum frame rate and maximum percentage of time for Green and Blue to achieve a decent White.

***Answer:** The minimum frame rate for a clear visual display and no flickering is 60 frames per second. The maximum percentage of time for all the colors (including Green and Blue) to achieve a decent white is 94%, although 75% is acceptable. Having a maximum percentage of 100% would be equivalent to a refresh rate of 160 fps, which is unnecessary. The difference between 94% on time and 100% on time is the 6% off-time that allows the CPU to rest.*

| Parameter                                         | Value                             |
|---------------------------------------------------|-----------------------------------|
| **Minimum frame rate for smooth display**         | **60 fps**                        |
| **Brightness level for decent white**             | **12/16 (75%)** - **15/16 (94%)** |
| **PWM tick time when refresh rate = 60 fps**      | **347.22 µs**                     |
| **maximumum on_time at level 15 to achive white** | **5.21 ms (94%)**                 |

---

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
  
5. **Answering Questions and Finalizations**
   - Performed on-time calculations for each brightness level
   - Created Table
   - Edited README 

---

## Reflection

This assignment went faster than expected because I front-loaded the time investment. Spending time to fully understand the program's structure and behavior made implementing and debugging much easier. I learned a lot about how the different components interact and how to approach modifying and extending existing code. The fun part was seeing the knob change the frame rate. Once I figured that out, I knew I was close to completing the if-else blocks that checked for button pressing that would change the individual LED colors. Eventually, I had to go back and re-read the assignment because I had forgotten to answer the question regarding the minimum frame rate and the maximum on-time percentage for each color to display a decent white. The assignment was fruitful and a valuable experience, covering all aspects of the microbit, including the GPIO board, connecting the LED to the GPIO board, and using wires to connect every component on the breadboard.

---

## Original README intact below. Created by my Professor [Bart Massey](https://github.com/pdx-cs-rust-embedded/hw-rgbcal-skeleton):

## rgbcal: RGB LED calibration tool
Bart Massey 2024

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

Please take a look below for UI.

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
