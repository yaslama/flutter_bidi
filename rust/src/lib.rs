use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use unic_bidi::{level, BidiInfo};

fn reorder_paras(text: &str) -> Vec<Cow<'_, str>> {
    let bidi_info = BidiInfo::new(text, Some(level::RTL_LEVEL));
    bidi_info
        .paragraphs
        .iter()
        .map(|para| bidi_info.reorder_line(para, para.range.clone()))
        .collect()
}

#[no_mangle]
pub extern "C" fn reorder(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let text = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let display = reorder_paras(text).join("\n");
    CString::new(display.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rust_cstr_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
