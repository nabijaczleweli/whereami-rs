use libc;


#[link(name = "whereami")]
extern "C" {
    fn wai_getExecutablePath(out: *mut u8,
                             capacity: libc::c_int,
                             dirname_length: *mut libc::c_int)
                             -> libc::c_int;
    fn wai_getModulePath(out: *mut u8,
                         capacity: libc::c_int,
                         dirname_length: *mut libc::c_int)
                         -> libc::c_int;
}

pub fn getExecutablePath(out: *mut u8,
                         capacity: libc::c_int,
                         dirname_length: *mut libc::c_int)
                         -> libc::c_int {
    unsafe { wai_getExecutablePath(out, capacity, dirname_length) }
}

pub fn getModulePath(out: *mut u8,
                     capacity: libc::c_int,
                     dirname_length: *mut libc::c_int)
                     -> libc::c_int {
    unsafe { wai_getModulePath(out, capacity, dirname_length) }
}
