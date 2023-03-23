use regex::{Captures, Regex};
use sqlite_loadable::{
    api,
    table::{ConstraintOperator, IndexInfo, VTab, VTabArguments, VTabCursor},
    BestIndexError, Result,
};
use sqlite_loadable::{prelude::*, Error};

use std::{mem, os::raw::c_int};

use crate::utils::{result_regex_captures, value_regex};

static CREATE_SQL: &str = "CREATE TABLE x(captures, pattern hidden, contents text hidden)";
enum Columns {
    Captures,
    Pattern,
    Contents,
}
fn column(index: i32) -> Option<Columns> {
    match index {
        0 => Some(Columns::Captures),
        1 => Some(Columns::Pattern),
        2 => Some(Columns::Contents),
        _ => None,
    }
}

#[repr(C)]
pub struct RegexCapturesTable {
    /// must be first
    base: sqlite3_vtab,
}

impl<'vtab> VTab<'vtab> for RegexCapturesTable {
    type Aux = ();
    type Cursor = RegexCapturesCursor<'vtab>;

    fn connect(
        _db: *mut sqlite3,
        _aux: Option<&Self::Aux>,
        _args: VTabArguments,
    ) -> Result<(String, RegexCapturesTable)> {
        let base: sqlite3_vtab = unsafe { mem::zeroed() };
        let vtab = RegexCapturesTable { base };
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
                Some(Columns::Pattern) => {
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
                _ => (),
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

    fn open(&mut self) -> Result<RegexCapturesCursor<'_>> {
        Ok(RegexCapturesCursor::new())
    }
}

#[repr(C)]
pub struct RegexCapturesCursor<'vtab> {
    /// Base class. Must be first
    base: sqlite3_vtab_cursor,
    r_clone: Option<Regex>,
    all_captures: Option<Vec<Captures<'vtab>>>,
    curr: usize,
}
impl RegexCapturesCursor<'_> {
    fn new<'vtab>() -> RegexCapturesCursor<'vtab> {
        let base: sqlite3_vtab_cursor = unsafe { mem::zeroed() };
        RegexCapturesCursor {
            base,
            r_clone: None,
            all_captures: None,
            curr: 0,
        }
    }
}

impl VTabCursor for RegexCapturesCursor<'_> {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        values: &[*mut sqlite3_value],
    ) -> Result<()> {
        let r = value_regex(
            values
                .get(0)
                .ok_or_else(|| Error::new_message("expected 1st argument as regex"))?,
        )?;
        let contents = api::value_text_notnull(
            values
                .get(1)
                .ok_or_else(|| Error::new_message("expected 2nd argument as contents"))?,
        )?;

        let mut res = vec![];
        for captures in r.captures_iter(contents) {
            res.push(captures)
        }
        self.r_clone = Some((*r).clone());
        Box::into_raw(r);
        self.all_captures = Some(res);
        self.curr = 0;
        Ok(())
    }

    fn next(&mut self) -> Result<()> {
        self.curr += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.all_captures
            .as_ref()
            .map_or(true, |m| self.curr >= m.len())
    }

    fn column(&self, context: *mut sqlite3_context, i: c_int) -> Result<()> {
        let captures = self
            .all_captures
            .as_ref()
            .ok_or_else(|| {
                Error::new_message("sqlite-regex internal error: self.all_captures is not defined")
            })?
            .get(self.curr)
            .ok_or_else(|| {
                Error::new_message(
                    "sqlite-regex internal error: self.curr greater than all_captures result",
                )
            })?;
        match column(i) {
            Some(Columns::Captures) => {
                result_regex_captures(context, self.r_clone.as_ref().unwrap(), captures);
            }
            Some(Columns::Pattern) => (),
            Some(Columns::Contents) => (),
            None => (),
        }
        Ok(())
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.curr as i64)
    }
}
