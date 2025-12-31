use crate::get;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn regex_specificity_get(
    string_ptr: *const core::ffi::c_char,
    pattern_ptr: *const core::ffi::c_char,
) -> i64 {
    if string_ptr.is_null() || pattern_ptr.is_null() {
        return 0;
    }

    let s = unsafe { core::ffi::CStr::from_ptr(string_ptr) };
    let p = unsafe { core::ffi::CStr::from_ptr(pattern_ptr) };

    let s_str = match s.to_str() {
        Ok(val) => val,
        Err(_) => return -1,
    };
    let p_str = match p.to_str() {
        Ok(val) => val,
        Err(_) => return -1,
    };

    match get(s_str, p_str) {
        Ok(score) => score as i64,
        Err(_) => -1,
    }
}
