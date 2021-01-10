pub use palette::{Hsl, Mix, Srgb};

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
}

/// Allow easy conversion an f32 tuple
/// Very useful for interfacing with palette
impl From<Color> for (f32, f32, f32) {
    fn from(color: Color) -> (f32, f32, f32) {
        (color.red as f32, color.green as f32, color.blue as f32)
    }
}

// Allow conversion to u8 tuple
impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> (u8, u8, u8) {
        (color.red, color.green, color.blue)
    }
}

/// Allow easy conversion from an f32 tuple
/// Very useful for interacting with palette
impl From<(f32, f32, f32)> for Color {
    fn from(components: (f32, f32, f32)) -> Color {
        Color {
            red: components.0 as u8,
            green: components.1 as u8,
            blue: components.2 as u8,
        }
    }
}

// Allow conversion from u8 tuple
impl From<(u8, u8, u8)> for Color {
    fn from(components: (u8, u8, u8)) -> Color {
        Color {
            red: components.0,
            green: components.1,
            blue: components.2,
        }
    }
}
