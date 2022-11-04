import sqlite3
import unittest
import time
import os

EXT_PATH="./target/debug/libregex0"

def connect(ext):
  db = sqlite3.connect(":memory:")

  db.execute("create table base_functions as select name from pragma_function_list")
  db.execute("create table base_modules as select name from pragma_module_list")

  db.enable_load_extension(True)
  db.load_extension(ext)

  db.execute("create temp table loaded_functions as select name from pragma_function_list where name not in (select name from base_functions) order by name")
  db.execute("create temp table loaded_modules as select name from pragma_module_list where name not in (select name from base_modules) order by name")

  db.row_factory = sqlite3.Row
  return db


db = connect(EXT_PATH)

def explain_query_plan(sql):
  return db.execute("explain query plan " + sql).fetchone()["detail"]

def execute_all(sql, args=None):
  if args is None: args = []
  results = db.execute(sql, args).fetchall()
  return list(map(lambda x: dict(x), results))

FUNCTIONS = [
  "regex_debug",
  "regex_find",
  "regex_replace",
  "regex_valid",
  "regex_version",
  "regexp",
]

MODULES = [
  "regex_find_all",
  "regex_split",
]
class TestRegex(unittest.TestCase):
  def test_funcs(self):
    funcs = list(map(lambda a: a[0], db.execute("select name from loaded_functions").fetchall()))
    self.assertEqual(funcs, FUNCTIONS)

  def test_modules(self):
    modules = list(map(lambda a: a[0], db.execute("select name from loaded_modules").fetchall()))
    self.assertEqual(modules, MODULES)
    
  def test_regex_version(self):
    version = 'v0.1.0'
    self.assertEqual(db.execute("select regex_version()").fetchone()[0], version)
  
  def test_regex_debug(self):
    debug = db.execute("select regex_debug()").fetchone()[0]
    self.assertEqual(len(debug.splitlines()), 2)
  
  def test_regexp(self):
    regexp = lambda pattern, content: db.execute("select regexp(?, ?)", [pattern, content]).fetchone()[0]
    self.assertEqual(regexp('^\d{4}-\d{2}-\d{2}$', '2022-01-01'), 1)
  
  def test_regex_valid(self):
    regex_valid = lambda pattern: db.execute("select regex_valid(?)", [pattern]).fetchone()[0]
    self.assertEqual(
      regex_valid("[0-9]{3}-[0-9]{3}-[0-9]{4}"),
      1
    )
    self.assertEqual(
      regex_valid("no("),
      0
    )

  def test_regex_find(self):
    regex_find = lambda pattern, content: db.execute("select regex_find(?, ?)", [pattern, content]).fetchone()[0]
    self.assertEqual(
      regex_find("[0-9]{3}-[0-9]{3}-[0-9]{4}", "phone: 111-222-3333"),
      '111-222-3333'
    )
    self.assertEqual(
      regex_find("[0-9]{3}-[0-9]{3}-[0-9]{4}", "phone: 111-222-333"),
      None
    )

    with self.assertRaisesRegex(sqlite3.OperationalError, "pattern not valid regex"):
      regex_find("[invalidregex", "abc")
    
  def test_regex_replace(self):
    regex_replace = lambda pattern, content, replacement: db.execute("select regex_replace(?, ?, ?)", [pattern, content, replacement]).fetchone()[0]
    
    self.assertEqual(
      regex_replace(
        '(?P<last>[^,\s]+),\s+(?P<first>\S+)',
        'Springsteen, Bruce',
        '$first $last'
      ),
      'Bruce Springsteen'
    )

    self.assertEqual(
      regex_replace(
        '(?P<first>\w+)\s+(?P<second>\w+)',
        'deep fried',
        '${first}_$second'
      ),
      'deep_fried'
    )
    #with self.assertRaisesRegex(sqlite3.OperationalError, "pattern not valid regex"):
    #  regex_find("[invalidregex", "abc")

  def test_regex_find_all(self):
    regex_find_all = lambda pattern, content: execute_all("select rowid, * from regex_find_all(?, ?)", [pattern, content])
    self.assertEqual(
      regex_find_all('\\b\w{13}\\b', 'Retroactively relinquishing remunerations is reprehensible.'),
      [
        {'rowid': 0, 'start': 0, 'end': 13, 'match': 'Retroactively',},
        {'rowid': 1, 'start': 14, 'end': 27, 'match': 'relinquishing',},
        {'rowid': 2, 'start': 28, 'end': 41, 'match': 'remunerations',},
        {'rowid': 3, 'start': 45, 'end': 58, 'match': 'reprehensible',}
      ]
    )
  
  def test_regex_split(self):
    regex_split = lambda pattern, content: execute_all("select rowid, * from regex_split(?, ?)", [pattern, content])
    self.assertEqual(
      regex_split('[ \t]+', 'a b \t  c\td    e'),
      [
        {'rowid': 0, 'item': 'a'},
        {'rowid': 1, 'item': 'b'},
        {'rowid': 2, 'item': 'c'},
        {'rowid': 3, 'item': 'd'},
        {'rowid': 4, 'item': 'e'}
      ]
    )

  
class TestCoverage(unittest.TestCase):                                      
  def test_coverage(self):                                                      
    test_methods = [method for method in dir(TestRegex) if method.startswith('test_')]
    funcs_with_tests = set([x.replace("test_", "") for x in test_methods])
    
    for func in FUNCTIONS:
      self.assertTrue(func in funcs_with_tests, f"{func} does not have corresponding test in {funcs_with_tests}")
    
    for module in MODULES:
      self.assertTrue(module in funcs_with_tests, f"{module} does not have corresponding test in {funcs_with_tests}")

if __name__ == '__main__':
    unittest.main()