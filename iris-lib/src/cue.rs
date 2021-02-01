use crate::color::Color;
use core::num::{NonZeroU16, NonZeroU8};
use fixed::types::U0F8; // 8-Bit fixed point number between 0 and 1
use serde;
use serde::{Deserialize, Serialize};

/// Number of RGB-LEDs. The total number of LEDs to drive is three times this.
pub const CHANNELS: u8 = 12;

/// The algorithm used for transitioning between two colors.
#[derive(Serialize, Deserialize)]
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

/// Newtype implementation of a fixed-point number x, where 0 ≤ x < 1
/// Will serialize into an [`f32`]
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(from = "f32")]
#[serde(into = "f32")]
pub struct RampRatio(U0F8);

impl From<RampRatio> for f32 {
    fn from(value: RampRatio) -> f32 {
        value.into()
    }
}

impl From<f32> for RampRatio {
    fn from(value: f32) -> RampRatio {
        RampRatio(U0F8::saturating_from_num(value))
    }
}

/// A simple animation that transitions between two colors cyclically.
/// It transitions from the start color to the end color and then back.
#[derive(Serialize, Deserialize)]
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
    /// - *1*: All LEDs are animated in the same manner
    pub time_divisor: NonZeroU8,
    /// The duration until the animation repeats.
    // u16 is enough for 65 seconds, we don't need more than that and it makes
    // sure the calculations don't overflow when applying the ramp ratio.
    pub duration_ms: NonZeroU16,
    /// The algorithm to use for transitioning between the two colors.
    /// Also see [`RampType`]
    pub ramp_type: RampType,
    /// The ratio between the transition from start to end and end to start between 0 and 1
    /// We use an 8-bit fixed point number as this gives a sufficient step size of
    /// ~0.004 and makes sure calculations don't overflow inside u32 registers
    pub ramp_ratio: RampRatio,
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
            time_divisor: NonZeroU8::new(CHANNELS).unwrap(),
            duration_ms: NonZeroU16::new(1000).unwrap(), // Don't set to 0, otherwise the Cue would be invisible
            ramp_type: RampType::Jump,
            ramp_ratio: 0.5.into(), // Don't set to 0, the start color would be invisible

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
            duration_ms: NonZeroU16::new(3000).unwrap(),
            ramp_type: RampType::LinearHSL { wrap_hue: false },
            ramp_ratio: 1.0.into(),
            start_color: Color::from_hsl(0, 100, 50),
            end_color: Color::from_hsl(359, 100, 50),
            ..Default::default()
        }
    }

    /// Create pre-built Cue displaying a clockwise rotating black and white half
    pub fn black_white_jump() -> Cue {
        Cue {
            duration_ms: NonZeroU16::new(3000).unwrap(),
            start_color: Color::white(),
            end_color: Color::black(),
            ..Default::default()
        }
    }

    /// Create pre-built Cue displaying a white breathing effect
    pub fn white_breathing() -> Cue {
        Cue {
            duration_ms: NonZeroU16::new(3600).unwrap(),
            ramp_type: RampType::LinearRGB,
            ramp_ratio: 0.4.into(),
            time_divisor: NonZeroU8::new(1).unwrap(),
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
        if progress <= self.ramp_ratio.0 {
            // Actual formula:
            // progress / ramp_ratio
            progress.saturating_div(self.ramp_ratio.0)
        } else {
            // We have to use saturating division here as well due to rounding errors
            // We also use U0F8::MAX instead of 1. Actual formula:
            // 1 - (progress - ramp_ratio)/(1 - ramp_ratio)
            U0F8::MAX - (progress - self.ramp_ratio.0).saturating_div(U0F8::MAX - self.ramp_ratio.0)
        }
    }

    // Calculate color for progress if the RampType was Jump
    fn color_jump(&self, progress: U0F8) -> Color {
        if progress < self.ramp_ratio.0 {
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
    /// use core::num::NonZeroU16
    ///
    /// let mut cue = Cue {
    ///     reverse: true,  // Reverse makes the numbers a little nicer
    ///     duration_ms: NonZeroU16::new(1200).unwrap(),
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

        // Handle reversed cue
        let channel = if self.reverse {
            channel
        } else {
            // In non-reverse, lower channels need a higher progress
            CHANNELS - 1 - channel
        };

        // We need the duration to be u32 in all calculations
        let duration = self.duration_ms.get() as u32;
        let time_divisor = self.time_divisor.get() as u32;

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
