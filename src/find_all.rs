use regex::Regex;

use sqlite3_loadable::{
    errors::{BestIndexError, Result},
    table::{ConstraintOperator, SqliteXIndexInfo, VTab, VTabCursor, VTableArguments},
    SqliteContext, SqliteValue,
};
use sqlite3ext_sys::{sqlite3, sqlite3_vtab, sqlite3_vtab_cursor};

use std::{marker::PhantomData, mem, os::raw::c_int, collections::HashMap, cell::{RefCell, RefMut}, rc::Rc};

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

type MMatch = (usize, usize, String);
#[repr(C)]
pub struct RegexFindAllCursor<'vtab> {
    /// Base class. Must be first
    base: sqlite3_vtab_cursor,
    cache:  RefMut<'vtab, HashMap<String, Box<Regex>>>,
    matches: Option<Vec<MMatch>>,
    curr: usize,
    phantom: PhantomData<&'vtab RegexFindAllTable>,
}
impl RegexFindAllCursor<'_> {
    fn new<'vtab>(cache:RefMut<'vtab, HashMap<String, Box<regex::Regex>>>) -> RegexFindAllCursor<'vtab> {
        let base: sqlite3_vtab_cursor = unsafe { mem::zeroed() };
        RegexFindAllCursor {
            base,
            cache,
            matches: None,
            curr: 0,
            phantom: PhantomData,
        }
    }
}

unsafe impl VTabCursor for RegexFindAllCursor<'_> {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        values: Vec<SqliteValue>,
    ) -> Result<()> {
        let pattern = values.get(0).unwrap().text()?;
        let contents = values.get(1).unwrap().text()?;

        let r = if let Some(regex) = self.cache.get(&pattern) {
          regex.to_owned()
        }else {
          let b = Box::new(Regex::new(&pattern).unwrap());
          println!("cache miss");
          self.cache.insert(pattern, b.clone());
          b
        };
        let mut res = vec![];
        for m in r.find_iter(contents.as_str()) {
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

    fn column(&self, ctx: SqliteContext, i: c_int) -> Result<()> {
        let m = self.matches.as_ref().unwrap().get(self.curr).unwrap();

        match column(i) {
            Some(Columns::Start) => {
                ctx.result_int(m.0.try_into().unwrap());
            }
            Some(Columns::End) => {
                ctx.result_int(m.1.try_into().unwrap());
            }
            Some(Columns::Match) => {
                ctx.result_text(&m.2)?;
            }
            _ => (),
        }
        Ok(())
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.curr as i64)
    }
}

#[repr(C)]
pub struct RegexFindAllTable {
    /// must be first
    base: sqlite3_vtab,
    cache:  Rc<RefCell<HashMap<String, Box<Regex>>>>,
}

unsafe impl<'vtab> VTab<'vtab> for RegexFindAllTable {
    type Aux = Rc<RefCell<HashMap<String, Box<Regex>>>>;
    type Cursor = RegexFindAllCursor<'vtab>;

    fn connect(
        _db: *mut sqlite3,
        aux: Option<&Self::Aux>,
        _args: VTableArguments,
    ) -> Result<(String, RegexFindAllTable)> {
        let base: sqlite3_vtab = unsafe { mem::zeroed() };
        let cache = aux.unwrap().to_owned();
        let vtab = RegexFindAllTable { base, cache };
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

    fn open(&mut self) -> Result<RegexFindAllCursor<'_>> {
      let x = self.cache.borrow_mut();
        Ok(RegexFindAllCursor::new(x))
    }
}
