use std::os::raw::{c_int, c_void};
use virtblocks::ultra::Ultra;

#[no_mangle]
pub extern "C" fn virtblocks_ultra_new() -> *mut Ultra<'static> {
    Box::into_raw(Box::new(Ultra::default()))
}

#[no_mangle]
pub extern "C" fn virtblocks_ultra_free(c_ultra: *mut Ultra) {
    std::mem::drop(unsafe { Box::from_raw(c_ultra) });
}

type UltraCb = unsafe extern "C" fn(c_int, *mut c_void);
type FreeCb = unsafe extern "C" fn(*mut c_void);

struct UltraCallback {
    cb: UltraCb,
    opaque: *mut c_void,
    free_cb: Option<FreeCb>,
}

impl Drop for UltraCallback {
    fn drop(&mut self) {
        match self.free_cb {
            Some(x) => unsafe { x(self.opaque) },
            None => return,
        };
    }
}

#[no_mangle]
pub extern "C" fn virtblocks_ultra_set_cb(
    c_ultra: *mut Ultra,
    callback: Option<UltraCb>,
    c_opaque: *mut c_void,
    free_cb: Option<FreeCb>,
) {
    let callback = match callback {
        Some(x) => x,
        None => return,
    };

    let rust_ultra = unsafe { &mut *c_ultra };

    let ultra_cb = UltraCallback {
        cb: callback,
        opaque: c_opaque,
        free_cb: free_cb,
    };

    rust_ultra.set_cb(move |x| unsafe {
        let UltraCallback {
            cb,
            opaque,
            free_cb: _,
        } = ultra_cb;
        cb(x, opaque)
    });
}

#[no_mangle]
pub extern "C" fn virtblocks_ultra_unset_cb(c_ultra: *mut Ultra) {
    assert!(!c_ultra.is_null());

    let rust_ultra = unsafe { &mut *c_ultra };

    rust_ultra.unset_cb();
}

#[no_mangle]
pub extern "C" fn virtblocks_ultra_call_me(c_ultra: *mut Ultra) {
    assert!(!c_ultra.is_null());

    let rust_ultra = unsafe { &mut *(c_ultra) };

    rust_ultra.call_me();
}
