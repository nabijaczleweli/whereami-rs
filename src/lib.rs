extern crate libc;

mod native;
mod implementation;

use std::path::PathBuf;


pub fn executable_path() -> Option<PathBuf> {
    implementation::get_path(native::wai_getExecutablePath)
}

pub fn module_path() -> Option<PathBuf> {
    implementation::get_path(native::wai_getModulePath)
}
