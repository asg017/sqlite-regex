use regex::{Regex, Split};

use sqlite3_loadable::{
    errors::{BestIndexError, Result},
    table::{ConstraintOperator, SqliteXIndexInfo, VTab, VTabCursor, VTableArguments},
    SqliteContext, SqliteValue,
};
use sqlite3ext_sys::{sqlite3, sqlite3_vtab, sqlite3_vtab_cursor};

use std::{marker::PhantomData, mem, os::raw::c_int};

static CREATE_SQL: &str =
    "CREATE TABLE x(item text, pattern hidden, contents text hidden)";
enum Columns {
    Item,
    Pattern,
    Contents,
}
fn column(index: i32) -> Option<Columns> {
    match index {
        0 => Some(Columns::Item),
        1 => Some(Columns::Pattern),
        2 => Some(Columns::Contents),
        _ => None,
    }
}



#[repr(C)]
pub struct RegexSplitTable {
    /// must be first
    base: sqlite3_vtab,
}

unsafe impl<'vtab> VTab<'vtab> for RegexSplitTable {
    type Aux = ();
    type Cursor = RegexSplitCursor<'vtab>;

    fn connect(
        _db: *mut sqlite3,
        _aux: Option<&()>,
        _args: VTableArguments,
    ) -> Result<(String, RegexSplitTable)> {
        let base: sqlite3_vtab = unsafe { mem::zeroed() };
        let vtab = RegexSplitTable { base };
        // TODO db.config(VTabConfig::Innocuous)?;
        Ok((CREATE_SQL.to_owned(), vtab))
    }
    fn destroy(&self) -> Result<()> {
        Ok(())
    }

    fn best_index(&self, mut info: SqliteXIndexInfo) -> core::result::Result<(), BestIndexError> {
        let mut has_pattern = false;
        let mut has_contents = false;
        for mut constraint in info.constraints() {
            match column(constraint.icolumn()) {
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

    fn open(&mut self) -> Result<RegexSplitCursor<'_>> {
        Ok(RegexSplitCursor::new())
    }
}


#[repr(C)]
pub struct RegexSplitCursor<'vtab> {
    /// Base class. Must be first
    base: sqlite3_vtab_cursor,
    split: Option<Vec<String>>,
    rowid: usize,
    phantom: PhantomData<&'vtab RegexSplitTable>,
}
impl RegexSplitCursor<'_> {
    fn new<'vtab>() -> RegexSplitCursor<'vtab> {
        let base: sqlite3_vtab_cursor = unsafe { mem::zeroed() };
        RegexSplitCursor {
            base,
            split: None,
            rowid: 0,
            phantom: PhantomData,
        }
    }
}

unsafe impl VTabCursor for RegexSplitCursor<'_> {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        values: Vec<SqliteValue>,
    ) -> Result<()> {
        let pattern = values.get(0).unwrap().text()?;
        let contents = values.get(1).unwrap().text()?;

        let r = Regex::new(&pattern).unwrap();
        let split = r.split(&contents);
        self.split = Some(split.map(|i| i.to_string()).collect());
        self.rowid = 0;
        Ok(())
    }

    fn next(&mut self) -> Result<()> {
        self.rowid += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.rowid >= self.split.as_ref().unwrap().len()
    }

    fn column(&self, ctx: SqliteContext, i: c_int) -> Result<()> {
        
        match column(i) {
            Some(Columns::Item) => {
                ctx.result_text(self.split.as_ref().unwrap().get(self.rowid).unwrap())?;
            }
            _ => (),
        }
        Ok(())
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.rowid.try_into().unwrap())
    }
}
