use regex::Regex;

use sqlite3_loadable::api::{
    context_auxdata_get, context_auxdata_set, context_result_bool, context_result_null,
    context_result_text, value_text, value_int,
};
use sqlite3_loadable::errors::{Error, Result};
use sqlite3ext_sys::{sqlite3_context, sqlite3_value};
use std::os::raw::c_void;

pub fn regexp(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (re, set_aux) = arg0_as_regex(context, values)?;

    let content = value_text(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?
            .to_owned(),
    )?;
    context_result_bool(context, re.as_ref().is_match(content));
    cleanup_arg0_regex(set_aux, context, re);
    Ok(())
}

pub fn regex_valid(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let arg_pattern = values
        .get(0)
        .ok_or_else(|| Error::new_message("expected 1st argument as pattern"))?
        .to_owned();
    let pattern = value_text(arg_pattern)?;

    let result = match Regex::new(pattern) {
        Ok(_) => true,
        Err(_) => false,
    };

    context_result_bool(context, result);
    Ok(())
}

pub fn regex_find(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
  let (re, set_aux) = arg0_as_regex(context, values)?;
  let arg_content = values
      .get(1)
      .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?
      .to_owned();

  let content = value_text(arg_content)?;
  match re.as_ref().find(content) {
      Some(m) => {
          context_result_text(context, m.as_str())?;
      }
      None => {
          context_result_null(context);
      }
  };

  cleanup_arg0_regex(set_aux, context, re);
  Ok(())
}

pub fn regex_find_at(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
  let (re, set_aux) = arg0_as_regex(context, values)?;
  let arg_content = values
      .get(1)
      .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?
      .to_owned();
      let arg_offset = values
      .get(1)
      .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?
      .to_owned();

  let content = value_text(arg_content)?;
  let offset = value_int(arg_offset) as usize;
  match re.as_ref().find_at(content, offset) {
      Some(m) => {
          context_result_text(context, m.as_str())?;
      }
      None => {
          context_result_null(context);
      }
  };

  cleanup_arg0_regex(set_aux, context, re);
  Ok(())
}

pub fn regex_replace(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
  let (re, set_aux) = arg0_as_regex(context, values)?;

  let content = value_text(
      values
          .get(1)
          .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?
          .to_owned(),
  )?;
  let replacement = value_text(
      values
          .get(2)
          .ok_or_else(|| Error::new_message("expected 3rd argument as replacement"))?
          .to_owned(),
  )?;
  let result = re.as_ref().replace(content, replacement);
  context_result_text(context, &result)?;

  cleanup_arg0_regex(set_aux, context, re);
  Ok(())
}

pub fn regex_replace_all(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
  let (re, set_aux) = arg0_as_regex(context, values)?;

  let content = value_text(
      values
          .get(1)
          .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?
          .to_owned(),
  )?;
  let replacement = value_text(
      values
          .get(2)
          .ok_or_else(|| Error::new_message("expected 3rd argument as replacement"))?
          .to_owned(),
  )?;
  let result = re.as_ref().replace_all(content, replacement);
  context_result_text(context, &result)?;

  cleanup_arg0_regex(set_aux, context, re);
  Ok(())
}

fn arg0_as_regex(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<(Box<Regex>, bool)> {
    let mut set_aux = false;

    let auxdata = context_auxdata_get(context, 0);
    let re = if auxdata.is_null() {
        set_aux = true;
        let arg_pattern = values
            .get(0)
            .ok_or_else(|| Error::new_message("expected 1st argument as pattern"))?
            .to_owned();
        let pattern = value_text(arg_pattern)?;

        let b = Box::new(
            Regex::new(pattern).map_err(|_| Error::new_message("pattern not valid regex"))?,
        );
        b
    } else {
        unsafe { Box::from_raw(auxdata as *mut Regex) }
    };

    Ok((re, set_aux))
}

unsafe extern "C" fn cleanup(_arg1: *mut c_void) {}

fn cleanup_arg0_regex(set_aux: bool, context: *mut sqlite3_context, re: Box<Regex>) {
    if set_aux {
        context_auxdata_set(
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
