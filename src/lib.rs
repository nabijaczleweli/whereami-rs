//! [`whereami`](https://github.com/gpakosz/whereami) for Rust.
//!
//! Used for checking where the current executable/DLL is.
//!
//! # Examples:
//!
//! ```
//! println!("This executable is at {}", whereami::executable_path().unwrap().to_str().unwrap());
//! println!("This exec/DLL is at {}", whereami::module_path().unwrap().to_str().unwrap());
//! ```


extern crate libc;

mod native;
mod implementation;

use std::path::PathBuf;


/// Returns the path of the current executable or `None` if acquiring thereof failed.
///
/// # Examples:
///
/// ```
/// # use whereami::executable_path;
/// println!("This executable is at {}", executable_path().unwrap().to_str().unwrap());
/// ```
pub fn executable_path() -> Option<PathBuf> {
    implementation::get_path(native::wai_getExecutablePath)
}

/// Returns the path of the current module or `None` if acquiring thereof failed.
///
/// Note: this is not the Rust `mod`, but rather an OS module
///       (most of the time: a DLL)
///
/// # Examples:
///
/// ```
/// # use whereami::module_path;
/// println!("This exec/DLL is at {}", module_path().unwrap().to_str().unwrap());
/// ```
pub fn module_path() -> Option<PathBuf> {
    implementation::get_path(native::wai_getModulePath)
}
