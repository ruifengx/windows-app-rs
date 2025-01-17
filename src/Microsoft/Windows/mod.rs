#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Windows_AppLifecycle")]
pub mod AppLifecycle;
#[cfg(feature = "Windows_ApplicationModel")]
pub mod ApplicationModel;
#[cfg(feature = "Windows_System")]
pub mod System;
