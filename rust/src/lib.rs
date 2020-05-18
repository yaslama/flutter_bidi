use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use unic_bidi::{BidiInfo, level};

#[no_mangle]
pub extern fn reorder(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let text = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let bidi_info = BidiInfo::new(&text, Some(level::RTL_LEVEL));
    let para = &bidi_info.paragraphs[0];
    let line = para.range.clone();
    let display = bidi_info.reorder_line(para, line);

    CString::new(display.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_cstr_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
