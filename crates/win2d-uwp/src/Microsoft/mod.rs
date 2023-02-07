#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
#[cfg(feature = "Graphics")]
pub mod Graphics;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
