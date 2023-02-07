#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
#[cfg(feature = "Graphics_Canvas")]
pub mod Canvas;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
