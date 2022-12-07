mod find_all;
mod meta;
mod regex;
mod regexset;
mod regexset_matches;
mod split;
mod utils;

use regexset_matches::RegexSetMatchesTable;
use sqlite_loadable::prelude::*;
use sqlite_loadable::{
    define_scalar_function, define_table_function, errors::Result, FunctionFlags,
};

use crate::{find_all::RegexFindAllTable, meta::*, regex::*, regexset::*, split::RegexSplitTable};

#[sqlite_entrypoint]
pub fn sqlite3_regex_init(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC;

    define_scalar_function(db, "regex_version", 0, regex_version, flags)?;
    define_scalar_function(db, "regex_debug", 0, regex_debug, flags)?;

    define_scalar_function(db, "regex", 1, regex, flags)?;
    define_scalar_function(db, "regex_print", 1, regex_print, flags)?;

    define_scalar_function(db, "regexp", 2, regexp, flags)?;

    define_scalar_function(db, "regex_valid", 1, regex_valid, flags)?;

    define_scalar_function(db, "regex_find", 2, regex_find, flags)?;
    define_scalar_function(db, "regex_find_at", 3, regex_find_at, flags)?;

    define_scalar_function(db, "regex_replace", 3, regex_replace, flags)?;
    define_scalar_function(db, "regex_replace_all", 3, regex_replace_all, flags)?;

    define_table_function::<RegexFindAllTable>(db, "regex_find_all", None)?;
    define_table_function::<RegexSplitTable>(db, "regex_split", None)?;

    define_scalar_function(db, "regexset", -1, regexset, flags)?;
    define_scalar_function(db, "regexset_print", 1, regexset_print, flags)?;
    define_scalar_function(db, "regexset_is_match", 2, regexset_is_match, flags)?;

    define_table_function::<RegexSetMatchesTable>(db, "regexset_matches", None)?;
    Ok(())
}
