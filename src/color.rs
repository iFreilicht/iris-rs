use az::Cast;
use fixed::types::U0F8;
use fixed_macro::types::U0F8;
pub use palette::{Hsl, Mix, Srgb};

/// Describes an RGB color. This is the format used for storing colors
#[derive(Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn from_hsl(h: u16, s: u8, l: u8) -> Color {
        let float_hsl = Hsl::new(h as f32, s as f32, l as f32);
        let float_rgb: Srgb = float_hsl.into();
        Color {
            red: float_rgb.red as u8,
            green: float_rgb.green as u8,
            blue: float_rgb.blue as u8,
        }
    }

    pub fn linear_mix_rgb(&self, other: &Color, factor: U0F8) {}
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

/// Allow conversion to any castable tuple
///
impl<T: Cast<T>> From<Color> for (T, T, T)
where
    u8: Cast<T>,
{
    fn from(color: Color) -> (T, T, T) {
        (
            Cast::cast(color.red),
            Cast::cast(color.green),
            Cast::cast(color.blue),
        )
    }
}

/// Allow easy conversion from any castable tuple
/// Very useful for interacting with palette
impl<T: Cast<u8>> From<(T, T, T)> for Color {
    fn from(components: (T, T, T)) -> Color {
        Color {
            red: Cast::cast(components.0),
            green: Cast::cast(components.1),
            blue: Cast::cast(components.2),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
