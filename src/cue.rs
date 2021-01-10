use crate::color::Color;

/// Number of RGB-LEDs. The total number of LEDs to drive is three times this.
pub const CHANNELS: u8 = 12;

/// The algorithm used for transitioning between two colors.
pub enum RampType {
    /// Hard cut, no interpolation between colors
    Jump,
    /// Interpolate R, G and B linearly. Will sometimes lead to ugly colors
    /// in between the two specified ones
    LinearRGB,
    /// Interpolate H, S and L linearly. Will always look nice, but may lead
    /// to undesired additional colors in between.
    LinearHSL {
        /// If true, jump over the gap between Hue 0° and 360°.
        /// For example, this allows to transition from yellow to pink through
        /// red, instead of through green, cyan and blue.
        wrap_hue: bool,
    },
}

/// A simple animation that transitions between two colors cyclically.
/// It transitions from the start color to the end color and then back.
pub struct Cue {
    /// Each LED can be turned off. This is only relevant when using the
    /// Cue in a Schedule
    pub channels: [bool; CHANNELS as usize],
    /// Play the Cue in reverse
    pub reverse: bool,
    /// Repeat the pattern after reaching a certain LED. Examples values:
    /// - *12*: One full rotation with no visible seams
    /// - *6*: Two moving elements with no visible seams
    /// - *4*: Three moving elements with no visible seams
    /// - *1*: Three moving elements with no visible seams
    pub time_divisor: u8,
    /// The duration until the animation repeats.
    // u16 is enough for 65 seconds, we don't need more than that and it makes
    // sure the calculations don't overflow when applying the ramp ratio.
    pub duration_ms: u16,
    /// The algorithm to use for transitioning between the two colors.
    /// Also see [`RampType`]
    pub ramp_type: RampType,
    /// The ratio between the transition from start to end and end to start
    /// as a fraction of 256.
    pub ramp_ratio: u8,
    pub start_color: Color,
    pub end_color: Color,
}

impl Cue {
    /// Create pre-built rainbow Cue
    pub fn rainbow() -> Cue {
        Cue {
            channels: [true; CHANNELS as usize],
            reverse: false,
            time_divisor: CHANNELS,
            duration_ms: 3000,
            ramp_type: RampType::LinearHSL { wrap_hue: false },
            ramp_ratio: u8::MAX,
            start_color: Color::from_hsl(0, 100, 50),
            end_color: Color::from_hsl(359, 100, 50),
        }
    }

    /// Calculate the Color of a single LED at a given point in time
    pub fn current_color(&self, time_ms: u32, channel: u8) -> Color {
        // Handle reversed cue
        let channel = if self.reverse {
            CHANNELS - 1 - channel
        } else {
            channel
        };
        // Offset calculation for given channel
        let time_ms = time_ms + ((self.duration_ms / self.time_divisor as u16) as u32 * channel as u32;
        // Make effect wrap around
        let time_ms = time_ms % self.duration_ms as u32;

        match self.ramp_type {
            RampType::Jump => self.jump_transition(time_ms),
            RampType::LinearRGB => self.linear_rgb_transition(time_ms),
            RampType::LinearHSL { .. } => self.linear_hsl_transition(time_ms),
        }
    }
}

/// Calculate the duration after which the ramp to the end color has to
/// be completed and the ramp back to the start color will be started
fn duration_threshold_ms(duration_ms: u16, ramp_ratio: u8) -> u32 {
    // All calculations have to be as u32 so they don't overflow
    let duration_ms = duration_ms as u32;
    let ramp_ratio = ramp_ratio as u32;

    // Scale up first, then scale back down for precision
    // The basic formula is just (ramp_ratio / 256) * duration_ms
    let scaled_up = duration_ms * ramp_ratio;
    scaled_up / (u8::MAX as u32)
}

/// Perform a linear transition between two numbers
pub fn linear_transition(start: u8, end: u8, duration_ms: u16, ramp_ratio: u8, time_ms: u32) -> u8 {
    // We use 32 bit ints to ensure high precision and prevent overflows
    let start = start as u32;
    let end = end as u32;

    // For each calculation, the result has to be a large positive integer,
    // so we always have to calculate with a positive delta
    let positive_delta = start < end;
    let delta = if positive_delta {
        end - start
    } else {
        start - end
    };

    let threshold_ms = duration_threshold_ms(duration_ms, ramp_ratio);

    let summand = if time_ms <= threshold_ms {
        // Upward slope
        (delta * time_ms) / threshold_ms
    } else {
        // Downward slope
        delta - (delta * (time_ms - threshold_ms)) / (duration_ms as u32 - threshold_ms)
    };

    if positive_delta {
        (start + summand) as u8
    } else {
        (start - summand) as u8
    }
}

#[cfg(test)]
mod test {
    use crate::cue::*;
    #[test]
    fn create_defaults() {
        let rainbow = Cue::rainbow();
    }
}
