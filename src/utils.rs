use regex::{Regex, RegexSet};

use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};

// Raw bytes as performance. the string MUST end in the null byte '\0'
const REGEX_POINTER_NAME: &[u8] = b"regex0\0";

pub fn value_regex(value: &*mut sqlite3_value) -> Result<Box<Regex>> {
    unsafe {
        if let Some(regex) = api::value_pointer(value, REGEX_POINTER_NAME) {
            return Ok(regex);
        }
    }
    let pattern = api::value_text_notnull(value)?;
    Ok(Box::new(Regex::new(pattern).map_err(|err| {
        Error::new_message(format!("Error parsing regex: {}", err).as_str())
    })?))
}

pub fn result_regex(context: *mut sqlite3_context, regex: Regex) {
    api::result_pointer(context, REGEX_POINTER_NAME, regex)
}

pub enum RegexInputType {
    Pointer,
    TextInitial(usize),
    GetAuxdata,
}
pub fn regex_from_value_or_cache(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    at: usize,
) -> Result<(Box<Regex>, RegexInputType)> {
    let value = values
        .get(at)
        .ok_or_else(|| Error::new_message("expected 1st argument as pattern"))?;

    // Step 1: If the value is a pointer result of regex(),
    // just use that.
    unsafe {
        if let Some(regex) = api::value_pointer(value, REGEX_POINTER_NAME) {
            return Ok((regex, RegexInputType::Pointer));
        }
    }

    // Step 2: If sqlite3_get_auxdata returns a pointer,
    // then use that.

    let auxdata = api::auxdata_get(context, at as i32);
    if !auxdata.is_null() {
        Ok((
            unsafe { Box::from_raw(auxdata.cast::<Regex>()) },
            RegexInputType::GetAuxdata,
        ))
    } else {
        // Step 3: if a string is passed in, then try to make
        // a regex from that, and return a flag to call sqlite3_set_auxdata

        let pattern = api::value_text_notnull(value)?;
        Ok((
            Box::new(
                Regex::new(pattern).map_err(|_| Error::new_message("pattern not valid regex"))?,
            ),
            RegexInputType::TextInitial(at),
        ))
    }
}

use std::os::raw::c_void;
unsafe extern "C" fn cleanup(_arg1: *mut c_void) {}

pub fn cleanup_regex_value_cached(
    context: *mut sqlite3_context,
    regex: Box<Regex>,
    input_type: RegexInputType,
) {
    let pointer = Box::into_raw(regex);
    match input_type {
        RegexInputType::Pointer => (),
        RegexInputType::GetAuxdata => {}
        RegexInputType::TextInitial(at) => {
            api::auxdata_set(
                context,
                at as i32,
                pointer.cast::<c_void>(),
                // TODO memory leak, box not destroyed?
                Some(cleanup),
            )
        }
    }
}

// Raw bytes as performance. the string MUST end in the null byte '\0'
const REGEX_SET_POINTER_NAME: &[u8] = b"regexset0\0";

pub fn value_regexset(value: &*mut sqlite3_value) -> Result<Box<RegexSet>> {
    unsafe {
        if let Some(regex) = api::value_pointer(value, REGEX_SET_POINTER_NAME) {
            return Ok(regex);
        }
    }
    Err(Error::new_message("asdf"))
}

pub fn result_regexset(context: *mut sqlite3_context, set: RegexSet) {
    api::result_pointer(context, REGEX_SET_POINTER_NAME, set)
}
