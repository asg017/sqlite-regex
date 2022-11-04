mod meta;
mod regexp;
mod find_all;
mod split;

use std::{collections::HashMap, cell::RefCell, rc::Rc};

use regex::Regex;
use sqlite3_loadable::{
    errors::Result, scalar::define_scalar_function, sqlite3_entrypoint, sqlite3_imports,
    table::define_table_function,
};
use sqlite3ext_sys::sqlite3;

use crate::{
    find_all::RegexFindAllTable,
    meta::{regex_debug, regex_version},
    regexp::{regex_find, regex_find_at, regex_replace, regex_replace_all, regex_valid, regexp},
    split::{RegexSplitTable},
};

sqlite3_imports!();

#[sqlite3_entrypoint]
pub fn sqlite3_regex_init(db: *mut sqlite3) -> Result<()> {
  let cache: HashMap<String, Box<Regex>> = HashMap::new();
  let c = Rc::new(RefCell::new(cache));

    define_scalar_function(db, "regex_version", 0, regex_version)?;
    define_scalar_function(db, "regex_debug", 0, regex_debug)?;

    define_scalar_function(db, "regexp", 2, regexp)?;

    define_scalar_function(db, "regex_valid", 1, regex_valid)?;
    define_scalar_function(db, "regex_find", 2, regex_find)?;
    //define_scalar_function(db, "regex_find_at", 3, regex_find_at)?;
    define_scalar_function(db, "regex_replace", 3, regex_replace)?;
    //define_scalar_function(db, "regex_replace_all", 3, regex_replace_all)?;

    define_table_function::<RegexFindAllTable>(db, "regex_find_all", Some(Rc::clone(&c)))?;
    define_table_function::<RegexSplitTable>(db, "regex_split", None)?;
    Ok(())
}
