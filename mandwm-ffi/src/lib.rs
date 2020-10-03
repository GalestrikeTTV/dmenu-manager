
//! # WARNING: This crate is currently unimplemented.
//! The purpose of this crate is to be a layer around the mandwm-api crate and produce C static
//! libs and dynamic libs.
//! 
//! Things to be done:
//!
//! * Generate C headers procedurally
//! * Autogen function table header/library for dylibs
//!
//! Pretty simple for a crate I would think. Might end up being a bit challenging with all the
//! xlib and dbus stuff. Might be better to write the ffi in C.

/// Obviously not the way things are going to be handled
pub fn gen_libs() { panic!("Please read the crate description before using the ffi.") }
