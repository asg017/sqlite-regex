use regex::Regex;

use crate::utils::{
    cleanup_regex_value_cached, regex_from_value_or_cache, result_regex, value_regex,
    value_regex_captures, CaptureGroupKey,
};
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};

// regex(pattern [, flags])
pub fn regex_print(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let regex = value_regex(values.get(0).ok_or("asdf")?)?;
    api::result_text(context, regex.as_str())?;
    Box::into_raw(regex);
    Ok(())
}

// regex(pattern)
pub fn regex(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let pattern = api::value_text_notnull(values.get(0).ok_or("")?)?;
    let regex = Regex::new(pattern).map_err(|err| {
        Error::new_message(format!("Error parsing pattern as regex: {}", err).as_str())
    })?;
    result_regex(context, regex);
    Ok(())
}

/// regex_matches(regex, text)
pub fn regex_matches(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (regex, input_type) = regex_from_value_or_cache(context, values, 0)?;
    let content =
        api::value_text_notnull(values.get(1).ok_or("expected 2nd argument as contents")?)?;

    api::result_bool(context, regex.as_ref().is_match(content));
    cleanup_regex_value_cached(context, regex, input_type);
    Ok(())
}

/// regexp(pattern, text) or text REGEXP pattern
// Alias of regex_matches
pub fn regexp(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    regex_matches(context, values)
}

/// regex_valid(pattern)
pub fn regex_valid(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let pattern = api::value_text_notnull(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("expected 1st argument as pattern"))?,
    )?;
    api::result_bool(context, Regex::new(pattern).is_ok());
    Ok(())
}

/// regex_find(regex, contents)
pub fn regex_find(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (regex, input_type) = regex_from_value_or_cache(context, values, 0)?;

    let arg_content = values
        .get(1)
        .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?;

    let content = api::value_text_notnull(arg_content)?;
    match regex.as_ref().find(content) {
        Some(m) => {
            api::result_text(context, m.as_str())?;
        }
        None => {
            api::result_null(context);
        }
    };

    cleanup_regex_value_cached(context, regex, input_type);
    Ok(())
}

/// regex_find_at(regex, contents, offset)
pub fn regex_find_at(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (regex, input_type) = regex_from_value_or_cache(context, values, 0)?;

    let arg_content = values
        .get(1)
        .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?;
    let arg_offset = values
        .get(1)
        .ok_or_else(|| Error::new_message("expected 2nd argument as offset"))?;

    let content = api::value_text_notnull(arg_content)?;
    let offset = api::value_int(arg_offset) as usize;
    match regex.as_ref().find_at(content, offset) {
        Some(m) => {
            api::result_text(context, m.as_str())?;
        }
        None => {
            api::result_null(context);
        }
    };

    cleanup_regex_value_cached(context, regex, input_type);

    Ok(())
}

/// regex_replace(regex, contents, replacement)
pub fn regex_replace(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (regex, input_type) = regex_from_value_or_cache(context, values, 0)?;

    let content = api::value_text_notnull(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?,
    )?;
    let replacement = api::value_text_notnull(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("expected 3rd argument as replacement"))?,
    )?;

    let result = regex.as_ref().replace(content, replacement);

    api::result_text(context, &result)?;
    cleanup_regex_value_cached(context, regex, input_type);

    Ok(())
}

/// regex_replace_all(regex, contents, replacement)
pub fn regex_replace_all(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let (regex, input_type) = regex_from_value_or_cache(context, values, 0)?;

    let content = api::value_text_notnull(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?,
    )?;
    let replacement = api::value_text_notnull(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("expected 3rd argument as replacement"))?,
    )?;
    let result = regex.as_ref().replace_all(content, replacement);
    api::result_text(context, &result)?;

    cleanup_regex_value_cached(context, regex, input_type);
    Ok(())
}

/// regex_capture(regex, contents, group)
pub fn regex_capture(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (regex, input_type) = regex_from_value_or_cache(context, values, 0)?;

    let content = api::value_text_notnull(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?,
    )?;
    let group_arg = values
        .get(2)
        .ok_or_else(|| Error::new_message("expected 3rd argument as group index or name"))?;

    let result = regex.as_ref().captures(content);
    match result {
        None => api::result_null(context),
        Some(captures) => {
            let matched_capture = match api::value_type(group_arg) {
                api::ValueType::Integer => captures.get(api::value_int64(group_arg) as usize),
                _ => {
                    let name = api::value_text(group_arg)?;
                    captures.name(name)
                }
            };
            match matched_capture {
                None => api::result_null(context),
                Some(matched_group) => {
                    api::result_text(context, matched_group.as_str())?;
                }
            }
        }
    }
    cleanup_regex_value_cached(context, regex, input_type);
    Ok(())
}

/// regex_capture(regex, contents, group)
pub fn regex_capture2(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let captures = value_regex_captures(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("expected 1st argument as capture group"))?,
    )?;
    let group_arg = values
        .get(1)
        .ok_or_else(|| Error::new_message("expected 3rd argument as group index or name"))?;

    let matched_capture = match api::value_type(group_arg) {
        api::ValueType::Integer => {
            let lookup = api::value_int64(group_arg) as usize;
            captures.iter().find(|c| {
                if let CaptureGroupKey::Index(idx) = c.key {
                    idx == lookup
                } else {
                    false
                }
            })
        }
        _ => {
            let name = api::value_text(group_arg)?;
            captures.iter().find(|c| {
                if let CaptureGroupKey::Name(n) = &c.key {
                    name == n
                } else {
                    false
                }
            })
        }
    };
    match matched_capture {
        None => api::result_null(context),
        Some(m) => match &m.value {
            Some(v) => api::result_text(context, v.as_str())?,
            None => api::result_null(context),
        },
    }
    Box::into_raw(captures);
    Ok(())
}
