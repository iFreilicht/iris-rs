use hex;
use iris_lib::color::Color;
use iris_lib::cue::Cue;
use std::sync::Arc;

pub struct Iris {
    cues: Vec<Arc<Cue>>,
    current: Option<Arc<Cue>>,
}

impl Iris {
    pub fn new() -> Iris {
        Iris {
            cues: Vec::new(),
            current: None,
        }
    }

    pub fn add_cue(&mut self) {
        self.cues.push(Arc::new(Cue::rainbow()));
    }
    pub fn delete_cue(&mut self, id: usize) {
        self.cues.remove(id);
    }
    pub fn launch_cue(&mut self, id: usize) {
        self.current = Some(self.cues[id].clone());
    }
    pub fn current_color(&self, time_ms: u32, channel: u8) -> String {
        match &self.current {
            Some(cue) => to_hex(cue.current_color(time_ms, channel)),
            None => "#000".into(),
        }
    }
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
