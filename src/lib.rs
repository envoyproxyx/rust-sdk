#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Define the init function for the module.
/// This macro should be used in the root of the module.
/// The code block passed to the macro will be executed exactly when the module is loaded.
///
/// ## Example
/// ```
/// use envoyx_rust_sdk::init;
///
/// init!({
///   println!("Hello, World!");
/// });
/// ```
///
#[macro_export]
macro_rules! init {
    ($code:block) => {
        #[no_mangle]
        pub extern "C" fn __envoy_dynamic_module_v1_event_program_init() {
            $code;
        }
    };
}
