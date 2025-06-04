/// This file deals with the potentiometer. The knob is used to
/// change the brightness level of a certain color in the RGB
/// scale depending on which button is pressed. The methods in the
/// struct read the voltage coming out of the knob, from 0 volts
/// to the maximum ~3.3 volts. Then the voltage value is cast into
/// the range between 0 and LEVEL-1.
use crate::*; // All crate methods and definitions are imported here.

/// Successive Approximation Analog-to-Digital Converter (saadc) peripheral.
/// It's a hardware component that converts analog signals into digital values.
/// https://docs.embassy.dev/embassy-nrf/git/nrf9160-ns/saadc/struct.Saadc.html#:~:text=impl%20Saadc&text=Continuous%20sampling%20on%20a%20single,sampling%20should%20continue%20or%20stop.
// 1-channel SAADC instance:
pub type Adc = saadc::Saadc<'static, 1>;

// ADC used to measure analog input is wrapped in a Knob struct:
pub struct Knob(Adc);

/// Implement public Knob methods for use in other crates.
/// Get the raw voltage value, cast it by clamping between
/// o and 3.27 volts, then smear the cast for smooth responsiveness
/// while diling the knob.
impl Knob {
    /// Create a new instance of a Knob struct to calibrate the Adc.
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await; // Calibrate saadc for accuracy.
        Self(adc) // Return the adc.
    }

    /// Set Knob value between 0 and Levels-1.
    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0]; // A single buffer for one sample.
        self.0.sample(&mut buf).await; // Async ADC sample.

        // Return 0 if value is less than 0, 0x7fff if value is larger
        // and cast into u16.
        /* Using a hex to decimal converter (https://www.rapidtables.com/convert/number/hex-to-decimal.html?x=7FFF)
        the value for 0x7fff is 32767 in decimal. */
        let raw = buf[0].clamp(0, 0x7fff) as u16;

        let scaled = raw as f32 / 10_000.0; // Scall the raw clamped value to be anywhere between 0 and 32767/10,000 ~ 3.27.

        // LEVELS defined in main.rs: pub const LEVELS: u32 = 16;
        // The result casts the scaled value in the range of LEVELS.
        /* Multiply scaled by (LEVELS + 2) cast into f32 for matching types,
        then subtract 2.0 as floating point. Then clamp... if the previous
        value is less than 0.0, then make it 0.0, if it is larger than
        (LEVELS - 1), the make it (LEVELS - 1) AKA clamp its value so that
        it is gauranteed to be between the top and bottom values that are
        clamping. Finally, floor the result to ensure whole number as u32: */
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32 // Return the value.
    }
}
