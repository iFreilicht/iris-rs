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

impl Default for Cue {
    /// Default Cue will be black, change at least the colors to make it visible!
    fn default() -> Cue {
        Cue {
            channels: [true; CHANNELS as usize],
            reverse: false,
            time_divisor: CHANNELS,
            duration_ms: 1000, // Don't set to 0, otherwise the Cue would be invisible
            ramp_type: RampType::Jump,
            ramp_ratio: U0F8!(0.5), // Don't set to 0, the start color would be invisible

            // Set colors to black, those should be changed!
            start_color: Color::black(),
            end_color: Color::black(),
        }
    }
}

impl Cue {
    /// Create pre-built Cue displaying a clockwise rotating rainbow
    pub fn rainbow() -> Cue {
        Cue {
            duration_ms: 3000,
            ramp_type: RampType::LinearHSL { wrap_hue: false },
            ramp_ratio: U0F8::MAX,
            start_color: Color::from_hsl(0, 100, 50),
            end_color: Color::from_hsl(359, 100, 50),
            ..Default::default()
        }
    }

    /// Create pre-built Cue displaying a clockwise rotating black and white half
    pub fn black_white_jump() -> Cue {
        Cue {
            duration_ms: 3000,
            start_color: Color::white(),
            end_color: Color::black(),
            ..Default::default()
        }
    }

    /// Create pre-built Cue displaying a white breathing effect
    pub fn white_breathing() -> Cue {
        Cue {
            duration_ms: 3600,
            ramp_type: RampType::LinearRGB,
            ramp_ratio: U0F8!(0.4),
            time_divisor: 1,
            start_color: Color::black(),
            end_color: Color::white(),
            ..Default::default()
        }
    }

    /// Calculate the Color of a single LED at a given point in time
    pub fn current_color(&self, time_ms: u32, channel: u8) -> Color {
        let progress = self.progress(time_ms, channel);

        match self.ramp_type {
            RampType::Jump => self.color_jump(progress),
            RampType::LinearRGB => self
                .start_color
                .linear_mix_rgb(&self.end_color, self.mixing_factor(progress)),
            RampType::LinearHSL { wrap_hue } => self.start_color.linear_mix_hsl(
                self.end_color,
                self.mixing_factor(progress),
                wrap_hue,
            ),
        }
    }

    // Calculate factor for color mixing
    fn mixing_factor(&self, progress: U0F8) -> U0F8 {
        // In theory, the maximum value that can occur is 1, but U0F8 can't represent that,
        // so we use saturating division, which prevents an overflow.
        if progress <= self.ramp_ratio {
            // Actual formula:
            // progress / ramp_ratio
            progress.saturating_div(self.ramp_ratio)
        } else {
            // We have to use saturating division here as well due to rounding errors
            // We also use U0F8::MAX instead of 1. Actual formula:
            // 1 - (progress - ramp_ratio)/(1 - ramp_ratio)
            U0F8::MAX - (progress - self.ramp_ratio).saturating_div(U0F8::MAX - self.ramp_ratio)
        }
    }

    // Calculate color for progress if the RampType was Jump
    fn color_jump(&self, progress: U0F8) -> Color {
        if progress < self.ramp_ratio {
            self.start_color
        } else {
            self.end_color
        }
    }

    /// Return a fraction of how far the animation has progressed for the specified LED
    /// # Examples:
    /// ```
    /// use iris_lib::color::Color;
    /// use iris_lib::cue::Cue;
    /// use fixed::types::U0F8;
    /// use fixed_macro::types::U0F8;
    ///
    /// let mut cue = Cue {
    ///     reverse: true,  // Reverse makes the numbers a little nicer
    ///     duration_ms: 1200,
    ///     time_divisor: 12,
    ///     .. Default::default()
    /// };
    ///
    /// assert_eq!(cue.progress(0,0), U0F8!(0));
    /// assert_eq!(cue.progress(600,0), U0F8!(0.5));
    /// assert_eq!(cue.progress(1199,0), U0F8::MAX);
    /// // Offsets each LED equally
    /// assert_eq!(cue.progress(0,2), cue.progress(600,8));
    /// assert_eq!(cue.progress(300,3), cue.progress(900,9));
    /// // wraps around
    /// assert_eq!(cue.progress(1200,0), 0);
    /// cue.time_divisor = 6;
    /// assert_eq!(cue.progress(200,1), cue.progress(200,7));
    /// ```
    pub fn progress(&self, time_ms: u32, channel: u8) -> U0F8 {
        assert!(channel < CHANNELS);

        // duration_ms cannot be 0, otherwise calculations below may underflow or cause a crash
        if self.duration_ms == 0 {
            return U0F8!(0);
        }

        // Handle reversed cue
        let channel = if self.reverse {
            channel
        } else {
            // In non-reverse, lower channels need a higher progress
            CHANNELS - 1 - channel
        };

        // We need the duration to be u32 in all calculations
        let duration = self.duration_ms as u32;
        let time_divisor = self.time_divisor as u32;

        // Offset calculation for given channel
        // `+ (time_divisor / 2)` achieves mathematical integer rounding, see https://stackoverflow.com/a/2422722/
        let time_ms = time_ms + (((duration * channel as u32) + (time_divisor / 2)) / time_divisor);

        // Make effect wrap around
        // As duration_ms is a u16, time_ms is now ≤ 0xFFFE
        let time_ms = time_ms % duration;

        // Calculate progress as a fraction of u8::MAX
        // The calculation for all maximum values that can be computed on here would be
        // 0xFFFE * 0xFF + 0xFFFE) / 0xFFFF = 0xFF
        // So the result will always fit into a u8.
        let progress_fraction = (time_ms * u8::MAX as u32 + duration / 2) / duration;
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
        let _ = Cue::white_breathing();
    }
}
