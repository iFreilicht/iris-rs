#![no_std]
use palette::{Hsl, Srgb, Mix};

const CHANNELS: u8 = 12; // Number of RGB-LEDs

#[derive(Copy, Clone)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn from_hsl(h: u16, s: u8, l: u8) -> Color {
        let float_hsl = Hsl::new(h as f32, s as f32, l as f32);
        let float_rgb: Srgb = float_hsl.into();
        Color {
            red: float_rgb.red as u8,
            green: float_rgb.green as u8,
            blue: float_rgb.blue as u8,
        }
    }
}

/// Allow easy conversion from a Color to an f32 tuple
/// Very useful for interfacing with palette
impl From<Color> for (f32, f32, f32){
    fn from(color: Color) -> (f32, f32, f32) {
        (color.red as f32, 
        color.green as f32,
    color.blue as f32)
    }
}

/// Allow easy conversion from an f32 tuple to a Color
/// Very useful for interacting with palette
impl From<(f32, f32, f32)> for Color{
    fn from(components: (f32, f32, f32)) -> Color {
        Color{
            red: components.0 as u8,
            green: components.1 as u8,
            blue: components.2 as u8
        }
    }
}

enum RampType {
    Jump,
    LinearRGB,
    LinearHSL { wrap_hue: bool },
}

struct Cue {
    channels: [bool; 12],
    reverse: bool,
    time_divisor: u8,
    duration_ms: u32,
    ramp_type: RampType,
    ramp_parameter: u32, // Maximum is equal to duration
    start_color: Color,
    end_color: Color,
}

impl Cue {
    /// Create pre-built rainbow Cue
    fn rainbow() -> Cue {
        Cue {
            channels: [true; CHANNELS as usize],
            reverse: false,
            time_divisor: CHANNELS,
            duration_ms: 3000,
            ramp_type: RampType::LinearHSL { wrap_hue: false },
            ramp_parameter: 3000,
            start_color: Color::from_hsl(0, 100, 50),
            end_color: Color::from_hsl(359, 100, 50),
        }
    }

    /// Perform a linear transition between two numbers
    /// TODO: Make this easier to read!
    fn linear_transition(&self, start: u8, end: u8, time_ms: u32) -> u8 {
        assert!(time_ms <= self.duration_ms);
        assert!(self.ramp_parameter <= self.duration_ms);

        // We use 32 bit ints for higher precision
        let start = start as u32;
        let end = end as u32;

        // For each calculation, the result has to be a large positive integer
        let delta: u32 = if start > end { start - end } else { end - start};
        let summand = if time_ms <= self.ramp_parameter {
            (delta * time_ms) / self.ramp_parameter 
        } else {
            delta - (delta * (time_ms - self.ramp_parameter))/(self.duration_ms - self.ramp_parameter)
        };
        
        if start > end {
            (start - summand) as u8
        } else {
            (start + summand) as u8
        }
    }

    /// Perform a linear transition in HSL space
    fn linear_hsl_transition(&self, time_ms: u32) -> Color {
        let start_hsl: Hsl = Srgb::from_components(self.start_color.into()).into();
        let end_hsl: Hsl = Srgb::from_components(self.end_color.into()).into();
        let new_hsl = start_hsl.mix(&end_hsl, (self.duration_ms / time_ms) as f32);
        let new_color: Color = Color::from(Srgb::from(new_hsl).into_components());
        new_color
    }

    fn linear_rgb_transition(&self, time_ms: u32) -> Color {
        Color {
            red: self.linear_transition(self.start_color.red, self.end_color.red, time_ms),
            green: self.linear_transition(self.start_color.green, self.end_color.green, time_ms),
            blue: self.linear_transition(self.start_color.blue, self.end_color.blue, time_ms),
        }
    }

    /// Calculate the Color of a single LED at a given point in time
    pub fn current_color(&self, time_ms: u32, channel: u8) -> Color {
        // Handle reversed cue
        let channel = if self.reverse { CHANNELS - 1 - channel } else { channel };
        
        // Offset calculation for given channel
        let time_ms = time_ms + (self.duration_ms / self.time_divisor as u32) * channel as u32;
        
        // Make effect wrap around
        let time_ms = time_ms % self.duration_ms;

        match self.ramp_type {
            RampType::Jump => {
                if time_ms < self.ramp_parameter {
                    self.start_color
                } else {
                    self.end_color
                }
            },
            RampType::LinearRGB => self.linear_rgb_transition(time_ms),
            RampType::LinearHSL { .. } => self.linear_hsl_transition(time_ms)
        }
    }
}

fn main() {
    let def_cue = Cue::rainbow();
    let tmp = def_cue.current_color(1200, 0);
}
