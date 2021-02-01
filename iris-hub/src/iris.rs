use core::num::{NonZeroU16, NonZeroU8};
use iris_lib::color::Color;
use iris_lib::cue::{Cue, CHANNELS};

use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default)]
pub struct Iris {
    cues: Vec<Arc<Mutex<Cue>>>,
    current: Option<Arc<Mutex<Cue>>>,
}

impl Iris {
    pub fn new() -> Iris {
        Iris::default()
    }

    pub fn add_cue(&mut self) {
        self.cues.push(Arc::new(Mutex::new(Cue::white_breathing())));
    }
    pub fn delete_cue(&mut self, id: usize) {
        self.cues.remove(id);
    }
    pub fn launch_cue(&mut self, id: usize) {
        self.current = Some(self.cues[id].clone());
    }
    pub fn num_cues(&self) -> usize {
        self.cues.len()
    }

    pub fn current_cue_id(&self) -> Option<usize> {
        match &self.current {
            Some(current) => Some(
                self.cues
                    .iter()
                    // Not finding current in cues would be a bug
                    .position(|cue| Arc::ptr_eq(&current, &cue))
                    // So we unwrap and re-wrap to catch if that ever happens
                    .unwrap(),
            ),
            None => None,
        }
    }

    pub fn current_color(&self, time_ms: u32, channel: u8) -> String {
        match &self.current {
            Some(cue) => to_hex(cue.lock().unwrap().current_color(time_ms, channel)),
            None => "#000".into(),
        }
    }
    /// Number of channels. Currently returns a constant value,
    /// but this may be changed in the future
    pub fn num_channels(&self) -> u8 {
        CHANNELS
    }

    // Define accessors for all fields of Cue
    define_accessors!(channels;
        channel(num: usize){ channels[num] } -> bool;
        // set_channel actually has the signature set_channel(num: usize, value: bool)
        set_channel(value){ channels[num] = value });
    define_accessors!(reverse() -> bool; set_reverse(value));
    define_accessors!(time_divisor;
        time_divisor(){time_divisor.get()} -> u8;
        set_time_divisor(value){*time_divisor = NonZeroU8::new(value).unwrap()});
    define_accessors!(duration_ms;
        duration_ms(){duration_ms.get()} -> u16;
        set_duration_ms(value){*duration_ms = NonZeroU16::new(value).unwrap()});
    define_accessors!(ramp_ratio() -> f32; set_ramp_ratio(value));
    define_accessors!(start_color;
        start_color(){to_hex(*start_color)}  -> String;
        set_start_color(value){*start_color = from_hex(value)});
    define_accessors!(end_color;
        end_color(){to_hex(*end_color)} -> String;
        set_end_color(value){*end_color = from_hex(value)});
}

/// Convert [`iris_lib::color::Color`] to a hex string
/// # Examples
/// ```
/// use iris_lib::color::Color;
/// use iris_hub::iris::to_hex;
/// assert_eq!(to_hex(Color::new(0,0,0)), "#000000");
/// assert_eq!(to_hex(Color::new(127,20,255)), "#7f14ff");
/// assert_eq!(to_hex(Color::new(255,100,38)), "#ff6426");
/// ```
pub fn to_hex(color: Color) -> String {
    let color_components: [u8; 3] = color.into();
    let mut output = String::new();
    output.push('#');
    output.push_str(&hex::encode(color_components));
    output
}

/// Convert hex string to [`iris_lib::color::Color`]
/// # Examples
/// ```
/// use iris_lib::color::Color;
/// use iris_hub::iris::from_hex;
/// assert_eq!(Color::new(0,0,0), from_hex("#000000".to_string()));
/// assert_eq!(Color::new(127,20,255), from_hex("#7f14ff".to_string()));
/// assert_eq!(Color::new(255,100,38), from_hex("#ff6426".to_string()));
/// ```
pub fn from_hex(string: String) -> Color {
    let mut str_buffer = string;
    str_buffer.remove(0);
    let color_vec = &hex::decode(str_buffer).unwrap();
    let components: [u8; 3] = [color_vec[0], color_vec[1], color_vec[2]];
    components.into()
}
