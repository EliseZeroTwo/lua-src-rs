use std::os::raw::{c_char, c_int, c_long, c_void};
#[cfg(feature = "sync")]
use std::sync::atomic::{AtomicUsize, Ordering};

extern "C" {
    pub fn luaL_newstate() -> *mut c_void;
    pub fn luaL_openlibs(state: *mut c_void);
    pub fn lua_getfield(state: *mut c_void, index: c_int, k: *const c_char);
    pub fn lua_tolstring(state: *mut c_void, index: c_int, len: *mut c_long) -> *const c_char;

    #[cfg(any(feature = "lua52", feature = "lua53", feature = "lua54"))]
    pub fn lua_getglobal(state: *mut c_void, k: *const c_char);
}

#[cfg(feature = "lua51")]
pub unsafe fn lua_getglobal(state: *mut c_void, k: *const c_char) {
    lua_getfield(state, -10002 /* LUA_GLOBALSINDEX */, k);
}

#[cfg(feature = "sync")]
static LOCK_COUNT: AtomicUsize = AtomicUsize::new(0);
#[cfg(feature = "sync")]
static UNLOCK_COUNT: AtomicUsize = AtomicUsize::new(0);

#[cfg(feature = "sync")]
#[no_mangle]
pub extern "C" fn lua_lock_rs(_state: *mut c_void) -> c_int {
    LOCK_COUNT.fetch_add(1, Ordering::Relaxed);
    0
}

#[cfg(feature = "sync")]
#[no_mangle]
pub extern "C" fn lua_unlock_rs(_state: *mut c_void) -> c_int {
    UNLOCK_COUNT.fetch_add(1, Ordering::Relaxed);
    0
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

        #[cfg(feature = "lua51")]
        assert_eq!(version, "Lua 5.1".as_bytes());
        #[cfg(feature = "lua52")]
        assert_eq!(version, "Lua 5.2".as_bytes());
        #[cfg(feature = "lua53")]
        assert_eq!(version, "Lua 5.3".as_bytes());
        #[cfg(feature = "lua54")]
        assert_eq!(version, "Lua 5.4".as_bytes());

        #[cfg(feature = "sync")]
        assert!(LOCK_COUNT.load(Ordering::Relaxed) > 0);
        #[cfg(feature = "sync")]
        assert_eq!(
            LOCK_COUNT.load(Ordering::Relaxed),
            UNLOCK_COUNT.load(Ordering::Relaxed)
        );
    }
}
