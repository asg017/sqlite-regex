use regex::RegexSet;
use sqlite_loadable::prelude::*;
use sqlite_loadable::{
    api,
    table::{ConstraintOperator, IndexInfo, VTab, VTabArguments, VTabCursor},
    BestIndexError, Result,
};

use std::{mem, os::raw::c_int};

use crate::utils::value_regexset;

static CREATE_SQL: &str = "CREATE TABLE x(x, regexset hidden, contents hidden)";
enum Columns {
    X,
    Regexset,
    Contents,
}
fn column(index: i32) -> Option<Columns> {
    match index {
        0 => Some(Columns::X),
        1 => Some(Columns::Regexset),
        2 => Some(Columns::Contents),
        _ => None,
    }
}

#[repr(C)]
pub struct RegexSetMatchesTable {
    /// must be first
    base: sqlite3_vtab,
}

impl<'vtab> VTab<'vtab> for RegexSetMatchesTable {
    type Aux = ();
    type Cursor = RegexSetMatchesCursor;

    fn connect(
        _db: *mut sqlite3,
        _aux: Option<&Self::Aux>,
        _args: VTabArguments,
    ) -> Result<(String, RegexSetMatchesTable)> {
        let base: sqlite3_vtab = unsafe { mem::zeroed() };
        let vtab = RegexSetMatchesTable { base };
        // TODO db.config(VTabConfig::Innocuous)?;
        Ok((CREATE_SQL.to_owned(), vtab))
    }
    fn destroy(&self) -> Result<()> {
        Ok(())
    }

    fn best_index(&self, mut info: IndexInfo) -> core::result::Result<(), BestIndexError> {
        let mut has_pattern = false;
        let mut has_contents = false;
        for mut constraint in info.constraints() {
            match column(constraint.column_idx()) {
                Some(Columns::Regexset) => {
                    if constraint.usable() && constraint.op() == Some(ConstraintOperator::EQ) {
                        constraint.set_omit(true);
                        constraint.set_argv_index(1);
                        has_pattern = true;
                    } else {
                        return Err(BestIndexError::Constraint);
                    }
                }
                Some(Columns::Contents) => {
                    if constraint.usable() && constraint.op() == Some(ConstraintOperator::EQ) {
                        constraint.set_omit(true);
                        constraint.set_argv_index(2);
                        has_contents = true;
                    } else {
                        return Err(BestIndexError::Constraint);
                    }
                }
                _ => todo!(),
            }
        }
        if !has_pattern || !has_contents {
            return Err(BestIndexError::Error);
        }
        info.set_estimated_cost(100000.0);
        info.set_estimated_rows(100000);
        info.set_idxnum(2);

        Ok(())
    }

    fn open(&mut self) -> Result<RegexSetMatchesCursor> {
        Ok(RegexSetMatchesCursor::new())
    }
}

#[repr(C)]
pub struct RegexSetMatchesCursor {
    /// Base class. Must be first
    base: sqlite3_vtab_cursor,
    regex_set: Option<RegexSet>,
    matches: Option<Vec<usize>>,
    rowid: usize,
}
impl RegexSetMatchesCursor {
    fn new() -> RegexSetMatchesCursor {
        let base: sqlite3_vtab_cursor = unsafe { mem::zeroed() };
        RegexSetMatchesCursor {
            base,
            regex_set: None,
            matches: None,
            rowid: 0,
        }
    }
}

impl VTabCursor for RegexSetMatchesCursor {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        values: &[*mut sqlite3_value],
    ) -> Result<()> {
        let r = value_regexset(values.get(0).unwrap())?;
        let contents = api::value_text(values.get(1).unwrap())?;
        self.regex_set = Some((*r).clone());
        self.matches = Some(r.matches(contents).into_iter().collect());
        Box::into_raw(r);
        Ok(())
    }

    fn next(&mut self) -> Result<()> {
        self.rowid += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.rowid >= self.matches.as_ref().unwrap().len()
    }

    fn column(&self, context: *mut sqlite3_context, i: c_int) -> Result<()> {
        let m = self.matches.as_ref().unwrap().get(self.rowid).unwrap();

        match column(i) {
            Some(Columns::X) => {
                //api::result_int(context, (*m).try_into().unwrap());
                let r = self.regex_set.as_ref().unwrap().patterns().get(*m).unwrap();
                api::result_text(context, r)?;
            }
            Some(Columns::Regexset) => {
                api::result_json(context, self.regex_set.as_ref().unwrap().patterns().into())?;
            }
            Some(Columns::Contents) => {}
            _ => (),
        }
        Ok(())
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.rowid as i64)
    }
}
