use libc;
use std::path::PathBuf;


pub fn get_path<F: FnOnce(*mut u8, libc::c_int, *mut libc::c_int) -> libc::c_int>(wai_func: F)
                                                                                  -> PathBuf {
    PathBuf::new()
}
