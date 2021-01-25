pub mod iris;
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
// TODO: Make a macro for this, it will get repetitive

#[wasm_bindgen]
pub fn add_cue() {
    IRIS.lock().unwrap().add_cue()
}

#[wasm_bindgen]
pub fn delete_cue(id: usize) {
    IRIS.lock().unwrap().delete_cue(id)
}

#[wasm_bindgen]
pub fn launch_cue(id: usize) {
    IRIS.lock().unwrap().launch_cue(id)
}

#[wasm_bindgen]
pub fn current_color(time_ms: u32, channel: u8) -> String {
    IRIS.lock().unwrap().current_color(time_ms, channel)
}
