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
    // Variant for complex case with field name, differently named
    // setter/getter statements and optional argument.
    ($field_name:ident; $getter:ident ($($arg:ident : $arg_t:ty)?){$from:stmt} -> $type:ty;
    $setter:ident($val:ident){$to:stmt}) => {
        /// Getter for $field_name, has the same name in most cases
        /// If no cue is active, the default value will be returned
        pub fn $getter(&self $(, $arg: $arg_t)?) -> $type {
            match &self.current {
                Some(current) => {
                    let $field_name = &current.lock().unwrap().$field_name;
                    $from
                },
                None => {
                    let $field_name = &Cue::default().$field_name;
                    $from
                },
            }
        }
        /// Setter for $field_name
        /// # Panics
        /// Panics if there is no current cue
        pub fn $setter(&mut self, $($arg : $arg_t ,)? $val: $type) {
            match &self.current {
                Some(current) => {
                    let $field_name =  &mut current.lock().unwrap().$field_name;
                    $to
                },
                _ => panic!("No cue is currently active!"),
            };
        }
    };
    // Generalized case where only the output type has to be specified
    ($field_name:ident () -> $type:ty; $setter:ident(value)) => {
        define_accessors!($field_name; 
                          $field_name(){ <$type>::from(*$field_name) } -> $type;
                          $setter(value){ *$field_name = value.into() });
    };
}
