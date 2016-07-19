use libc;


#[link(name = "whereami")]
extern "C" {
    pub fn wai_getExecutablePath(out: *mut u8,
                                 capacity: libc::c_int,
                                 dirname_length: *mut libc::c_int)
                                 -> libc::c_int;
    pub fn wai_getModulePath(out: *mut u8,
                             capacity: libc::c_int,
                             dirname_length: *mut libc::c_int)
                             -> libc::c_int;
}
