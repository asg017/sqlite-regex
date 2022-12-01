use regex::{Regex, RegexSet};

use sqlite_loadable::api::ValueType;
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};

// Raw bytes as performance. the string MUST end in the null byte '\0'
const REGEX_POINTER_NAME: &[u8] = b"regex0\0";
const REGEX_SET_POINTER_NAME: &[u8] = b"regexset0\0";

pub fn value_regex(value: &*mut sqlite3_value) -> Result<Box<Regex>> {
  unsafe {
      if let Some(regex) = api::value_pointer(value, REGEX_POINTER_NAME) {
          return Ok(regex);
      }
  }
  let pattern = api::value_text_notnull(value)?;
  Ok(Box::new(
      Regex::new(pattern).map_err(|_| Error::new_message("asdf"))?,
  ))
}


//pub fn value_r(value: &*mut sqlite3_value) -> Result<Box<Regex>> {}

pub fn result_regex(context: *mut sqlite3_context, regex: Regex) {
    api::result_pointer(context, REGEX_POINTER_NAME, regex)
}

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
