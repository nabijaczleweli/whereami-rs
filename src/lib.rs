extern crate libc;

mod native;
mod implementation;

use std::path::PathBuf;


pub fn executable_path() -> PathBuf {
    implementation::get_path(native::getExecutablePath )
}

pub fn module_path() -> PathBuf {
    implementation::get_path(native::getModulePath )
}
