use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ops::Range;
use unic_bidi::{level, BidiInfo, ParagraphInfo};
mod bidi_mirroring_glyph_table;
use crate::bidi_mirroring_glyph_table::bidi_mirroring_glyph;

pub trait BidiInfoExt<'text> {
    fn full_reorder_line(&self, para: &ParagraphInfo, line: Range<usize>) -> Cow<'text, str>;
}

impl<'text> BidiInfoExt<'text> for BidiInfo<'text> {
    /// Re-order a line based on resolved levels and return the line in display order.
    fn full_reorder_line(&self, para: &ParagraphInfo, line: Range<usize>) -> Cow<'text, str> {
        let (levels, runs) = self.visual_runs(para, line.clone());

        // If all isolating run sequences are LTR, no reordering is needed
        if runs.iter().all(|run| levels[run.start].is_ltr()) {
            return self.text[line.clone()].into();
        }

        let mut result = String::with_capacity(line.len());
        for run in runs {
            if levels[run.start].is_rtl() {
                result.extend(self.text[run].chars().rev().map(|c| {
                    match bidi_mirroring_glyph(c as u32) {
                        Some(n) => match std::char::from_u32(n.get()) {
                            Some(r) => r,
                            None => c,
                        },
                        None => c,
                    }
                }));
            } else {
                result.push_str(&self.text[run]);
            }
        }
        result.into()
    }
}

fn reorder_paras(text: &str) -> Vec<Cow<'_, str>> {
    let bidi_info = BidiInfo::new(text, Some(level::RTL_LEVEL));
    bidi_info
        .paragraphs
        .iter()
        .map(|para| bidi_info.full_reorder_line(para, para.range.clone()))
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
