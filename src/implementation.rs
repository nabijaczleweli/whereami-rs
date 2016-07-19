use libc;
use std::ptr::null_mut;
use std::path::PathBuf;


#[cfg(windows)]
static MISPADDING: usize = 1;

#[cfg(not(windows))]
static MISPADDING: usize = 0;


pub type NativeFunc = unsafe extern "C" fn(*mut u8, libc::c_int, *mut libc::c_int) -> libc::c_int;


pub fn get_path(wai_func: NativeFunc) -> Option<PathBuf> {
    let path_len = match unsafe { wai_func(null_mut(), 0, null_mut()) } {
        -1 => return None,
        len => len,
    };
    let mut path: Vec<u8> = Vec::with_capacity(path_len as usize - MISPADDING);

    path.resize(path_len as usize - MISPADDING, 0);
    unsafe { wai_func(path.as_mut_ptr(), path.len() as libc::c_int, null_mut()) };

    String::from_utf8(path).ok().map(|s| PathBuf::from(s))
}
