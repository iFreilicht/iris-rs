pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Create a wasm binding for a function in the iris object
/// by accessing the singleton IRIS and forwarding all arguments
macro_rules! bind_from_iris {
    // With return type
    ($func_name:ident($($arg:ident: $arg_t:ty),*) $(-> $ret_t:ty)?) => {
        #[wasm_bindgen]
        pub fn $func_name( $($arg: $arg_t),* ) $(-> $ret_t)? {
            IRIS.lock().unwrap().$func_name($($arg),*)
        }
    };
}

/// Implement a getter and setter for the specified fields of the current cue
/// `$type` is the type to convert to/from, not the one stored inside [`Cue`]
#[macro_use]
macro_rules! define_accessors {
    ($field_name:ident() -> $type:ty; $setter:ident()) => {
        // Getter for $field_name, has the same name
        // If no cue is active, the default value will be returned
        pub fn $field_name(&self) -> $type {
            match &self.current {
                Some(current) => current.lock().unwrap().$field_name.into(),
                None => Cue::default().$field_name.into(),
            }
        }
        // Setter for $field_name
        // # Panics
        // Panics if there is no current cue
        pub fn $setter(&mut self, value: $type) {
            match &self.current {
                Some(current) => current.lock().unwrap().$field_name = value.into(),
                _ => panic!("No cue is currently active!"),
            };
        }
    };
}
