use std::os::raw::{c_char, c_int, c_long, c_void};

extern "C" {
    pub fn luaL_newstate() -> *mut c_void;
    pub fn luaL_openlibs(state: *mut c_void);
    pub fn lua_getfield(state: *mut c_void, index: c_int, k: *const c_char);
    pub fn lua_tolstring(state: *mut c_void, index: c_int, len: *mut c_long) -> *const c_char;

    #[cfg(feature = "picolua54")]
    pub fn lua_getglobal(state: *mut c_void, k: *const c_char);
}

#[test]
fn lua_works() {
    use std::{ptr, slice};
    unsafe {
        let state = luaL_newstate();
        assert!(state != ptr::null_mut());

        luaL_openlibs(state);

        let version = {
            lua_getglobal(state, "_VERSION\0".as_ptr().cast());
            let mut len: c_long = 0;
            let version_ptr = lua_tolstring(state, -1, &mut len);
            slice::from_raw_parts(version_ptr as *const u8, len as usize)
        };

        #[cfg(feature = "picolua54")]
        assert_eq!(version, "PicoLua 5.4".as_bytes());
    }
}
