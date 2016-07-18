extern crate libc;

mod native;
mod implementation;

use std::path::PathBuf;


pub fn executable_path() -> PathBuf {
    implementation::get_path(native::get_executable_path)
}

pub fn module_path() -> PathBuf {
    implementation::get_path(native::get_module_path)
}
