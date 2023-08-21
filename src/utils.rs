use regex::{Captures, Regex, RegexSet};

use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};

// Raw bytes as performance. the string MUST end in the null byte '\0'
const REGEX_POINTER_NAME: &[u8] = b"regex0\0";

pub fn value_regex(value: &*mut sqlite3_value) -> Result<*mut Regex> {
    unsafe {
        if let Some(regex) = api::value_pointer(value, REGEX_POINTER_NAME) {
            return Ok(regex);
        }
    }
    let pattern = api::value_text_notnull(value)?;
    let x = Box::new(
        Regex::new(pattern)
            .map_err(|err| Error::new_message(format!("Error parsing regex: {}", err).as_str()))?,
    );
    Ok(Box::into_raw(x))
}

pub fn result_regex(context: *mut sqlite3_context, regex: Regex) {
    api::result_pointer(context, REGEX_POINTER_NAME, regex)
}

pub(crate) enum CaptureGroupKey {
    Index(usize),
    Name(String),
}

pub(crate) struct CaptureGroup {
    pub key: CaptureGroupKey,
    pub value: Option<String>,
}
const REGEX_CAPTURES_NAME: &[u8] = b"regex_captures0\0";

pub(crate) fn value_regex_captures(value: &*mut sqlite3_value) -> Result<*mut Vec<CaptureGroup>> {
    unsafe {
        if let Some(capture) = api::value_pointer(value, REGEX_CAPTURES_NAME) {
            return Ok(capture);
        }
    }
    Err(Error::new_message("value is not a regex captures object"))
}

pub fn result_regex_captures(context: *mut sqlite3_context, regex: &Regex, captures: &Captures) {
    let mut caps: Vec<CaptureGroup> = vec![];
    for name in regex.capture_names().flatten() {
        caps.push(CaptureGroup {
            key: CaptureGroupKey::Name(name.to_string()),
            value: captures.name(name).map(|v| v.as_str().to_string()),
        })
    }
    for (i, m) in captures.iter().enumerate() {
        caps.push(CaptureGroup {
            key: CaptureGroupKey::Index(i),
            value: m.map(|v| v.as_str().to_string()),
        })
    }
    api::result_pointer(context, REGEX_CAPTURES_NAME, caps)
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
) -> Result<(*mut Regex, RegexInputType)> {
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
        Ok((auxdata.cast::<Regex>(), RegexInputType::GetAuxdata))
    } else {
        // Step 3: if a string is passed in, then try to make
        // a regex from that, and return a flag to call sqlite3_set_auxdata

        let pattern = api::value_text_notnull(value)?;
        let boxed = Box::new(
            Regex::new(pattern).map_err(|_| Error::new_message("pattern not valid regex"))?,
        );
        Ok((Box::into_raw(boxed), RegexInputType::TextInitial(at)))
    }
}

use std::os::raw::c_void;
unsafe extern "C" fn cleanup(_arg1: *mut c_void) {}

pub fn cleanup_regex_value_cached(
    context: *mut sqlite3_context,
    regex: *mut Regex,
    input_type: RegexInputType,
) {
    match input_type {
        RegexInputType::Pointer => (),
        RegexInputType::GetAuxdata => {}
        RegexInputType::TextInitial(at) => {
            api::auxdata_set(
                context,
                at as i32,
                regex.cast::<c_void>(),
                // TODO memory leak, box not destroyed?
                Some(cleanup),
            )
        }
    }
}

// Raw bytes as performance. the string MUST end in the null byte '\0'
const REGEX_SET_POINTER_NAME: &[u8] = b"regexset0\0";

pub fn value_regexset(value: &*mut sqlite3_value) -> Result<*mut RegexSet> {
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
