use sqlite_loadable::prelude::*;
use sqlite_loadable::{
    api,
    table::{ConstraintOperator, IndexInfo, VTab, VTabArguments, VTabCursor},
    BestIndexError, Result,
};

use std::{marker::PhantomData, mem, os::raw::c_int};

use crate::utils::value_regex;

static CREATE_SQL: &str =
    "CREATE TABLE x(start int, end int, match text, pattern hidden, contents text hidden)";
enum Columns {
    Start,
    End,
    Match,
    Pattern,
    Contents,
}
fn column(index: i32) -> Option<Columns> {
    match index {
        0 => Some(Columns::Start),
        1 => Some(Columns::End),
        2 => Some(Columns::Match),
        3 => Some(Columns::Pattern),
        4 => Some(Columns::Contents),
        _ => None,
    }
}

#[repr(C)]
pub struct RegexFindAllTable {
    /// must be first
    base: sqlite3_vtab,
}

impl<'vtab> VTab<'vtab> for RegexFindAllTable {
    type Aux = ();
    type Cursor = RegexFindAllCursor<'vtab>;

    fn connect(
        _db: *mut sqlite3,
        _aux: Option<&Self::Aux>,
        _args: VTabArguments,
    ) -> Result<(String, RegexFindAllTable)> {
        let base: sqlite3_vtab = unsafe { mem::zeroed() };
        let vtab = RegexFindAllTable { base };
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

    fn open(&mut self) -> Result<RegexFindAllCursor<'_>> {
        Ok(RegexFindAllCursor::new())
    }
}

type MMatch = (usize, usize, String);
#[repr(C)]
pub struct RegexFindAllCursor<'vtab> {
    /// Base class. Must be first
    base: sqlite3_vtab_cursor,
    matches: Option<Vec<MMatch>>,
    curr: usize,
    phantom: PhantomData<&'vtab RegexFindAllTable>,
}
impl RegexFindAllCursor<'_> {
    fn new<'vtab>() -> RegexFindAllCursor<'vtab> {
        let base: sqlite3_vtab_cursor = unsafe { mem::zeroed() };
        RegexFindAllCursor {
            base,
            matches: None,
            curr: 0,
            phantom: PhantomData,
        }
    }
}

impl VTabCursor for RegexFindAllCursor<'_> {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        values: &[*mut sqlite3_value],
    ) -> Result<()> {
        let r = value_regex(values.get(0).unwrap())?;
        let contents = api::value_text(values.get(1).unwrap())?;

        let mut res = vec![];
        for m in r.find_iter(contents) {
            res.push((m.start(), m.end(), m.as_str().to_string()))
        }
        self.matches = Some(res);
        Ok(())
    }

    fn next(&mut self) -> Result<()> {
        self.curr += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.curr >= self.matches.as_ref().unwrap().len()
    }

    fn column(&self, context: *mut sqlite3_context, i: c_int) -> Result<()> {
        let m = self.matches.as_ref().unwrap().get(self.curr).unwrap();

        match column(i) {
            Some(Columns::Start) => {
                api::result_int(context, m.0.try_into().unwrap());
            }
            Some(Columns::End) => {
                api::result_int(context, m.1.try_into().unwrap());
            }
            Some(Columns::Match) => {
                api::result_text(context, &m.2)?;
            }
            _ => (),
        }
        Ok(())
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.curr as i64)
    }
}
