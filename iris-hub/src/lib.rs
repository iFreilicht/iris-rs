pub mod iris;
#[macro_use]
mod utils;

use iris::Iris;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A singleton carrying the global state that all getter and setter functions
// exported by [`wasm_bindgen`] access. This way, JS never holds a handle to our
// objects, giving us more freedom about which types to use internally
// Additionally, this prevents copying unnecessary data between JS and WASM
static IRIS: Lazy<Mutex<Iris>> = Lazy::new(|| Mutex::new(Iris::new()));

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

// Create bindings to functions defined inside [`Iris`]
bind_from_iris!(add_cue());
bind_from_iris!(delete_cue(id: usize));
bind_from_iris!(launch_cue(id: usize));
bind_from_iris!(current_color(time_ms: u32, channel: u8) -> String);
