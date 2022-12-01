use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Result};

pub fn regex_version(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(context, &format!("v{}", env!("CARGO_PKG_VERSION")))?;
    Ok(())
}

pub fn regex_debug(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(
        context,
        &format!(
            "Version: v{}
Source: {}
",
            env!("CARGO_PKG_VERSION"),
            env!("GIT_HASH")
        ),
    )?;
    Ok(())
}
