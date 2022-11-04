use sqlite3_loadable::{api::context_result_text, errors::Result, sqlite3_context, sqlite3_value};

pub fn regex_version(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    context_result_text(context, &format!("v{}", env!("CARGO_PKG_VERSION")))?;
    Ok(())
}

pub fn regex_debug(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    context_result_text(
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
