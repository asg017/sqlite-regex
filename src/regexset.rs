use regex::RegexSet;

use crate::utils::{result_regexset, value_regexset};
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};

pub fn regexset(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let mut patterns = Vec::with_capacity(values.len());
    for value in values {
        let pattern = api::value_text_notnull(value)?;
        patterns.push(pattern);
    }
    let set = RegexSet::new(patterns).map_err(|_| Error::new_message("asdf"))?;
    result_regexset(context, set);
    Ok(())
}

pub fn regexset_print(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let regexset = value_regexset(values.get(0).ok_or_else(|| Error::new_message(""))?)?;
    api::result_json(context, regexset.patterns().into())?;
    Box::into_raw(regexset);
    Ok(())
}

pub fn regexset_is_match(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let regexset = value_regexset(values.get(0).ok_or_else(|| Error::new_message(""))?)?;
    let text = api::value_text(values.get(1).ok_or_else(|| Error::new_message(""))?)?;
    api::result_bool(context, regexset.is_match(text));
    Box::into_raw(regexset);
    Ok(())
}
