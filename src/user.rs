#[derive(Clone, Debug)]
pub struct User {
    uid: u32,
    gid: u32,
}

impl User {
    fn new(uid: u32, gid: u32) -> Self {
        Self { uid, gid }
    }

    pub fn as_user(name: Option<String>) -> Option<Self> {
        match name {
            Some(username) => get_user_by_name(&username),
            None => None,
        }
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> u32 {
        self.gid
    }
}

mod libc {
    use std::ffi::{c_char, c_int};

    #[repr(C)]
    pub struct Passwd {
        pub pw_name: *mut c_char,
        pub pw_passwd: *mut c_char,
        pub pw_uid: u32,
        pub pw_gid: u32,
        pub pw_gecos: *mut c_char,
        pub pw_dir: *mut c_char,
        pub pw_shell: *mut c_char,
    }

    extern "C" {
        pub fn getpwnam_r(
            name: *const c_char,
            pwd: *mut Passwd,
            buf: *mut c_char,
            buflen: usize,
            result: *mut *mut Passwd,
        ) -> c_int;
    }
}

#[allow(unsafe_code)]
fn get_user_by_name(username: &str) -> Option<User> {
    let username = std::ffi::CString::new(username.as_bytes()).ok()?;

    // SAFETY: safe because we pass a concrete struct
    let mut passwd = unsafe { std::mem::zeroed::<libc::Passwd>() };
    let mut buf = vec![0; 2048];
    let mut result = std::ptr::null_mut::<libc::Passwd>();

    // SAFETY: safe because all pointers are initialized
    let ok = unsafe {
        libc::getpwnam_r(
            username.as_ptr(),
            &mut passwd,
            buf.as_mut_ptr(),
            buf.len(),
            &mut result,
        )
    };

    if ok != 0 || result.is_null() || result != &mut passwd {
        return None;
    }

    // SAFETY: safe because we checked that the result is valid
    let result = unsafe { result.read() };
    Some(User::new(result.pw_uid, result.pw_gid))
}
