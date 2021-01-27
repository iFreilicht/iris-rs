use crate::color::Color;
use fixed::types::U0F8; // 8-Bit fixed point number between 0 and 1
use fixed_macro::types::U0F8;

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
    /// The ratio between the transition from start to end and end to start between 0 and 1
    /// We use an 8-bit fixed point number as this gives a sufficient step size of
    /// ~0.004 and makes sure calculations don't overflow inside u32 registers
    pub ramp_ratio: U0F8,
    /// The color to start from
    pub start_color: Color,
    /// The color to transition to
    pub end_color: Color,
}

impl Cue {
    /// Create pre-built Cue displaying a clockwise rotating rainbow
    pub fn rainbow() -> Cue {
        Cue {
            channels: [true; CHANNELS as usize],
            reverse: false,
            time_divisor: CHANNELS,
            duration_ms: 3000,
            ramp_type: RampType::LinearHSL { wrap_hue: false },
            ramp_ratio: U0F8::MAX,
            start_color: Color::from_hsl(0, 100, 50),
            end_color: Color::from_hsl(359, 100, 50),
        }
    }

    /// Create pre-built Cue displaying a clockwise rotating black and white half
    pub fn black_white_jump() -> Cue {
        Cue {
            channels: [true; CHANNELS as usize],
            reverse: false,
            time_divisor: CHANNELS,
            duration_ms: 3000,
            ramp_type: RampType::Jump,
            ramp_ratio: U0F8!(0.5),
            start_color: Color::white(),
            end_color: Color::black(),
        }
    }

    /// Calculate the Color of a single LED at a given point in time
    pub fn current_color(&self, time_ms: u32, channel: u8) -> Color {
        let progress = self.get_progress(time_ms, channel);

        match self.ramp_type {
            RampType::Jump => self.get_color_jump(progress),
            RampType::LinearRGB => self.start_color.linear_mix_rgb(&self.end_color, progress),
            RampType::LinearHSL { wrap_hue } => {
                self.start_color
                    .linear_mix_hsl(self.end_color, progress, wrap_hue)
            }
        }
    }

    // Calculate color for progress if the RampType was Jump
    fn get_color_jump(&self, progress: U0F8) -> Color {
        if progress < self.ramp_ratio {
            self.start_color
        } else {
            self.end_color
        }
    }

    // Return a fraction of how far the animation has progressed for the specified LED
    fn get_progress(&self, time_ms: u32, channel: u8) -> U0F8 {
        assert!(channel < CHANNELS);

        // Handle reversed cue
        // TODO: why is the if-statement this way around so reverse means counter-clockwise?
        let channel = if self.reverse {
            channel
        } else {
            CHANNELS - 1 - channel
        };

        // Offset calculation for given channel
        let time_ms =
            time_ms + ((self.duration_ms / self.time_divisor as u16) as u32 * channel as u32);

        // Make effect wrap around
        // As duration_ms is a u16, time_ms will now fit into a u16 as well
        let time_ms = time_ms % self.duration_ms as u32;

        // Calculate progress as a fraction of u8::MAX
        // Because time_ms ≤ duration_ms, the result will be ≤ u8::MAX, so the cast below will succeed
        let progress_fraction = (time_ms * u8::MAX as u32) / self.duration_ms as u32;
        U0F8::from_bits(progress_fraction as u8)
    }
}

#[cfg(test)]
mod test {
    use crate::cue::*;
    #[test]
    fn create_defaults() {
        let _ = Cue::rainbow();
        let _ = Cue::black_white_jump();
    }
}
