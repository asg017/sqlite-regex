use regex::Regex;

use crate::utils::{result_regex, value_regex};
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};
use std::os::raw::c_void;

// regex(pattern [, flags])
pub fn regex_print(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let regex = value_regex(values.get(0).ok_or("asdf")?)?;
    api::result_text(context, regex.as_str())?;
    Box::into_raw(regex);
    Ok(())
}

// regex(pattern)
pub fn regex(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let pattern = api::value_text_notnull(values.get(0).ok_or("asdf")?)?;
    let regex = Regex::new(pattern).map_err(|e| Error::new_message("asdf"))?;
    result_regex(context, regex);
    Ok(())
}

/// regex_matches(pattern, text)
pub fn regex_matches(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
  //let (re, set_aux) = arg_as_cached_regex(context, values, 0)?;
  let re = value_regex(values.get(0).unwrap())?;

  let content = api::value_text_notnull(values.get(1).ok_or("expected 2nd argument as contents")?)?;
  api::result_bool(context, re.as_ref().is_match(content));
  Box::into_raw(re);
  //cleanup_arg0_regex(set_aux, context, re);
  Ok(())
}

/// regexp(pattern, text) or text REGEXP pattern
pub fn regexp(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
  regex_matches(context, values)
}

pub fn regex_valid(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let arg_pattern = values
        .get(0)
        .ok_or_else(|| Error::new_message("expected 1st argument as pattern"))?;
    let pattern = api::value_text_notnull(arg_pattern)?;
    api::result_bool(context, Regex::new(pattern).is_ok());
    Ok(())
}

pub fn regex_find(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (re, set_aux) = arg_as_cached_regex(context, values, 0)?;
    let arg_content = values
        .get(1)
        .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?;

    let content = api::value_text(arg_content)?;
    match re.as_ref().find(content) {
        Some(m) => {
            api::result_text(context, m.as_str())?;
        }
        None => {
            api::result_null(context);
        }
    };

    cleanup_arg0_regex(set_aux, context, re);
    Ok(())
}

pub fn regex_find_at(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (re, set_aux) = arg_as_cached_regex(context, values, 0)?;
    let arg_content = values
        .get(1)
        .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?;
    let arg_offset = values
        .get(1)
        .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?;

    let content = api::value_text(arg_content)?;
    let offset = api::value_int(arg_offset) as usize;
    match re.as_ref().find_at(content, offset) {
        Some(m) => {
            api::result_text(context, m.as_str())?;
        }
        None => {
            api::result_null(context);
        }
    };

    cleanup_arg0_regex(set_aux, context, re);
    Ok(())
}

pub fn regex_replace(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (re, set_aux) = arg_as_cached_regex(context, values, 0)?;

    let content = api::value_text(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?,
    )?;
    let replacement = api::value_text(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("expected 3rd argument as replacement"))?,
    )?;
    let result = re.as_ref().replace(content, replacement);
    api::result_text(context, &result)?;

    cleanup_arg0_regex(set_aux, context, re);
    Ok(())
}

pub fn regex_replace_all(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let (re, set_aux) = arg_as_cached_regex(context, values, 0)?;

    let content = api::value_text(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?,
    )?;
    let replacement = api::value_text(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("expected 3rd argument as replacement"))?,
    )?;
    let result = re.as_ref().replace_all(content, replacement);
    api::result_text(context, &result)?;

    cleanup_arg0_regex(set_aux, context, re);
    Ok(())
}
/*
fn arg0_as_regexx(
    context: *mut sqlite3_context,
    values: &Vec<SqliteValue>,
) -> Result<(Box<Regex>, bool)> {
    let mut set_aux = false;

    let auxdata = context_auxdata_get(context, 0);
    let re = if auxdata.is_null() {
        set_aux = true;
        let arg_pattern = values
            .get(0)
            .ok_or_else(|| Error::new_message("expected 1st argument as pattern"))?
            .to_owned();
        let pattern = arg_pattern.text()?;

        let b = Box::new(
            Regex::new(&pattern).map_err(|_| Error::new_message("pattern not valid regex"))?,
        );
        b
    } else {
        unsafe { Box::from_raw(auxdata as *mut Regex) }
    };

    Ok((re, set_aux))
}
*/

fn arg_as_cached_regex(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    at: i32,
) -> Result<(Box<Regex>, bool)> {
    let mut set_aux = false;

    let auxdata = api::auxdata_get(context, at);
    let re = if auxdata.is_null() {
        set_aux = true;
        let arg_pattern = values
            .get(0)
            .ok_or_else(|| Error::new_message("expected 1st argument as pattern"))?;
        let pattern = api::value_text(arg_pattern)?;

        Box::new(Regex::new(pattern).map_err(|_| Error::new_message("pattern not valid regex"))?)
    } else {
        unsafe { Box::from_raw(auxdata.cast::<Regex>()) }
    };

    Ok((re, set_aux))
}
unsafe extern "C" fn cleanup(_arg1: *mut c_void) {}

fn cleanup_arg0_regex(set_aux: bool, context: *mut sqlite3_context, re: Box<Regex>) {
    if set_aux {
        api::auxdata_set(
            context,
            0,
            Box::into_raw(re).cast::<c_void>(),
            // TODO memory leak, box not destroyed?
            Some(cleanup),
        );
    } else {
        Box::into_raw(re);
    }
}

fn regex_from_value_at_or_cache() {}
