use hex;
use iris_lib::color::Color;
use iris_lib::cue::{Cue, CHANNELS};

use std::sync::Arc;
use std::sync::Mutex;

pub struct Iris {
    cues: Vec<Arc<Mutex<Cue>>>,
    current: Option<Arc<Mutex<Cue>>>,
}

impl Iris {
    pub fn new() -> Iris {
        Iris {
            cues: Vec::new(),
            current: None,
        }
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

    define_accessors!(reverse, set_reverse -> bool);
    define_accessors!(time_divisor, set_time_divisor -> u8);
    define_accessors!(duration_ms, set_duration_ms -> u16);
    define_accessors!(start_color, set_start_color -> Color);
    define_accessors!(end_color, set_end_color -> Color);
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
