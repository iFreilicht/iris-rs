use fixed::types::U0F8;
pub use palette::{Hsl, Mix, Srgb};
use serde::{Deserialize, Serialize};

/// Describes an RGB color. This is the format used for storing colors
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn black() -> Color {
        [0, 0, 0].into()
    }

    pub fn white() -> Color {
        [255, 255, 255].into()
    }

    pub fn from_hsl(h: u16, s: u8, l: u8) -> Color {
        let float_hsl = Hsl::new(h as f32, s as f32, l as f32);
        float_hsl.into()
    }

    pub fn linear_mix_rgb(&self, other: &Color, factor: U0F8) -> Color {
        Color {
            red: interpolate(self.red, other.red, factor),
            green: interpolate(self.green, other.green, factor),
            blue: interpolate(self.blue, other.blue, factor),
        }
    }

    pub fn linear_mix_hsl(self, other: Color, factor: U0F8, _wrap_hue: bool) -> Color {
        // TODO: Interpolate values manually and implement wrap_hue
        // Maybe as a separate function, mix likely does something else than linear interpolation
        let self_hsl: Hsl = self.into();
        let other_hsl: Hsl = other.into();

        let new_hsl = self_hsl.mix(&other_hsl, factor.to_num());

        new_hsl.into()
    }
}

/// Interpolate between two numbers using a fixed-point factor between 0 and 1
/// # Examples
/// ```
/// use iris_lib::color::interpolate;
/// use fixed::types::U0F8;
/// use fixed_macro::types::U0F8;
///
/// assert_eq!(interpolate(0, 200, U0F8!(0.5)), 100);
/// assert_eq!(interpolate(200, 100, U0F8!(0.2)), 180);
/// assert_eq!(interpolate(0, 99, U0F8!(0.333)), 33);
/// assert_eq!(interpolate(20, 120, U0F8!(0.5)), 70);
///
/// // U0F8 can hold any x where 0 ≤ x < 1, so use MAX instead of 1
/// assert_eq!(interpolate(33, 250, U0F8::MAX), 250);
/// assert_eq!(interpolate(199, 5, U0F8::MAX), 5);
/// ```
pub fn interpolate(start: u8, end: u8, factor: U0F8) -> u8 {
    // We work with unsigned integers, so we need to make sure our delta is positive
    let positive_delta = start < end;
    let delta = if positive_delta {
        end - start
    } else {
        start - end
    };

    // As both factors are u8, the output of the multiplication will fit into u16
    // After dividing by 256, it fits exactly into a u8 again.
    // This is mathematically guaranteed, but can't be checked statically
    let scaled_summand = ((delta as u16 * factor.to_bits() as u16) / u8::MAX as u16) as u8;

    if positive_delta {
        start + scaled_summand
    } else {
        start - scaled_summand
    }
}

impl From<Hsl> for Color {
    fn from(hsl: Hsl) -> Color {
        let float_rgb: Srgb = hsl.into();
        let u8_rgb: Srgb<u8> = float_rgb.into_format();
        u8_rgb.into()
    }
}

impl From<Color> for Hsl {
    fn from(color: Color) -> Hsl {
        let u8_rgb: Srgb<u8> = color.into();
        let float_rgb: Srgb = u8_rgb.into_format();
        float_rgb.into()
    }
}

/// Allow conversion to palette's integer RGB type
impl From<Color> for Srgb<u8> {
    fn from(color: Color) -> Srgb<u8> {
        Srgb::<u8>::from_components((color.red, color.green, color.blue))
    }
}

/// Allow conversion from palette's integer RGB type
impl From<Srgb<u8>> for Color {
    fn from(u8_rgb: Srgb<u8>) -> Color {
        let (red, green, blue) = u8_rgb.into_components();
        Color { red, green, blue }
    }
}

/// Allow conversion to iterable array. Useful for converting to hex
impl From<Color> for [u8; 3] {
    fn from(color: Color) -> [u8; 3] {
        [color.red, color.green, color.blue]
    }
}

/// Allow conversion from array
impl From<[u8; 3]> for Color {
    fn from(arr: [u8; 3]) -> Color {
        Color {
            red: arr[0],
            green: arr[1],
            blue: arr[2],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fixed_macro::types::U0F8;
    #[test]
    fn test_interpolate() {
        // Trivial cases
        assert_eq!(interpolate(0, 255, U0F8::MIN), 0);
        assert_eq!(interpolate(0, 255, U0F8::MAX), 255);
        // Flipped inputs
        assert_eq!(interpolate(255, 0, U0F8::MIN), 255);
        assert_eq!(interpolate(255, 0, U0F8::MAX), 0);

        // Fractions of 255
        assert_eq!(interpolate(0, 255, U0F8!(0.75)), 192);
        assert_eq!(interpolate(0, 255, U0F8!(0.5)), 128);
        assert_eq!(interpolate(0, 255, U0F8!(0.25)), 64);
    }
}
