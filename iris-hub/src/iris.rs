use iris_lib::cue::Cue;
use std::sync::Arc;

pub struct Iris{
    cues: Vec<Arc<Cue>>,
    current: Option<Arc<Cue>>
}

impl Iris{
    pub fn new() -> Iris{
        Iris{
            cues: Vec::new(),
            current: None
        }
    }

    pub fn add_cue(&mut self){
        self.cues.push(Arc::new(Cue::rainbow()));
    }
    pub fn delete_cue(&mut self, id: usize){
        self.cues.remove(id);
    }
    pub fn launch_cue(&mut self, id: usize){
        self.current = Some(self.cues[id].clone());
    }
    pub fn current_color(&self, time_ms: u32, channel: u8) -> String{
        let color_components: [u8; 3] = match &self.current{
            Some(cue) => cue.current_color(time_ms, channel).into(),
            None => [0, 0, 0]
        };
        let mut output = String::new();
        output.push('#');
        output.push_str(&hex::encode(color_components));
        output
    }
}