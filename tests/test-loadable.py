import sqlite3
import unittest
import time
import os

EXT_PATH="./dist/debug/regex0"

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
  "regex",
  "regex_capture",
  "regex_capture",
  "regex_debug",
  "regex_find",
  "regex_find_at",
  "regex_print",
  "regex_replace",
  "regex_replace_all",
  "regex_valid",
  "regex_version",
  "regexp",
  "regexset",
  "regexset_is_match",
  "regexset_print",
]

MODULES = [
  "regex_captures",
  "regex_find_all",
  "regex_split",
  "regexset_matches",
]
def spread_args(args):
  return ",".join(['?'] * len(args))

class TestRegex(unittest.TestCase):
  def test_funcs(self):
    funcs = list(map(lambda a: a[0], db.execute("select name from loaded_functions").fetchall()))
    self.assertEqual(funcs, FUNCTIONS)

  def test_modules(self):
    modules = list(map(lambda a: a[0], db.execute("select name from loaded_modules").fetchall()))
    self.assertEqual(modules, MODULES)

  def test_regex_version(self):
    self.assertEqual(db.execute("select regex_version()").fetchone()[0][0], "v")

  def test_regex_debug(self):
    debug = db.execute("select regex_debug()").fetchone()[0]
    self.assertEqual(len(debug.splitlines()), 2)

  def test_regex(self):
    regex = lambda pattern: db.execute("select regex(?)", [pattern]).fetchone()[0]
    self.assertEqual(regex('^\d{4}-\d{2}-\d{2}$'), None)

    with self.assertRaisesRegex(sqlite3.OperationalError, "Error parsing pattern as regex: regex parse error:.*"):
      regex("[nope")


  def test_regex_print(self):
    regex_print = lambda pattern: db.execute("select regex_print(regex(?))", [pattern]).fetchone()[0]
    self.assertEqual(regex_print('^\d{4}-\d{2}-\d{2}$'), '^\d{4}-\d{2}-\d{2}$')

  def test_regexset(self):
    regexset = lambda *patterns: db.execute("select regexset({args})".format(args=spread_args(patterns)), patterns).fetchone()[0]
    self.assertEqual(regexset('a'), None)
    self.assertEqual(regexset('a', 'b'), None)

  def test_regexset_print(self):
    regexset_print = lambda *patterns: db.execute("select regexset_print(regexset({args}))".format(args=spread_args(patterns)), patterns).fetchone()[0]
    self.assertEqual(regexset_print('a', 'b', 'c'), '["a","b","c"]')

  def test_regexset_is_match(self):
    regexset_is_match = lambda *patterns, text: db.execute("select regexset_is_match(regexset({args}), ?)".format(args=spread_args(patterns)), [*patterns, text]).fetchone()[0]
    self.assertEqual(regexset_is_match('a', text='bbb'), 0)
    self.assertEqual(regexset_is_match('a', 'b', text='ccc'), 0)
    self.assertEqual(regexset_is_match('a', 'b', text='ccca'), 1)
    self.assertEqual(regexset_is_match('a', 'b', text='cccb'), 1)

  def test_regexset_matches(self):
    regexset_matches = lambda *patterns, text: execute_all("select rowid, * from regexset_matches(regexset({args}), ?)".format(args=spread_args(patterns)), [*patterns, text])
    self.assertEqual(
      regexset_matches('x', 'y', 'z', 'a', 'b', text='cab'),
      [
        {'rowid': 0, 'key': 3, 'pattern': 'a'},
        {'rowid': 1, 'key': 4, 'pattern': 'b'}
      ]
    )

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


  def test_regex_find_at(self):
    regex_find_at = lambda pattern, content, offset: db.execute("select regex_find_at(?, ?, ?)", [pattern, content, offset]).fetchone()[0]
    self.assertEqual(
      regex_find_at("[0-9]{3}-[0-9]{3}-[0-9]{4}", "phone: 111-222-3333", 0),
      '111-222-3333'
    )
    with self.assertRaisesRegex(sqlite3.OperationalError, "pattern not valid regex"):
      regex_find_at("[invalidregex", "abc", 0)

  def test_regex_capture(self):
    regex_capture = lambda pattern, content, group: db.execute("select regex_capture(?, ?, ?)", [pattern, content, group]).fetchone()[0]
    MOVIE_PATTERN = "'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)"
    EXAMPLE1 = "Not my favorite movie: 'Citizen Kane' (1941)."
    self.assertEqual(
      regex_capture(MOVIE_PATTERN, EXAMPLE1, 0),
      "'Citizen Kane' (1941)"
    )
    self.assertEqual(
      regex_capture(MOVIE_PATTERN, EXAMPLE1, 1),
      "Citizen Kane"
    )
    self.assertEqual(
      regex_capture(MOVIE_PATTERN, EXAMPLE1, 2),
      "1941"
    )
    self.assertEqual(
      regex_capture(MOVIE_PATTERN, EXAMPLE1, "title"),
      "Citizen Kane"
    )
    self.assertEqual(
      regex_capture(MOVIE_PATTERN, EXAMPLE1, "year"),
      "1941"
    )
    self.assertEqual(
      regex_capture(MOVIE_PATTERN, EXAMPLE1, "not exist"),
      None
    )
    self.assertEqual(
      regex_capture(MOVIE_PATTERN, EXAMPLE1, 3),
      None
    )
    self.assertEqual(
      regex_capture(MOVIE_PATTERN, EXAMPLE1, 1.1),
      None
    )
    self.assertEqual(
      regex_capture(MOVIE_PATTERN, EXAMPLE1, None),
      None
    )

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
    self.assertEqual(
      regex_replace('a', 'abc abc', ''),
      'bc abc'
    )

    #with self.assertRaisesRegex(sqlite3.OperationalError, "pattern not valid regex"):
    #  regex_find("[invalidregex", "abc")

  def test_regex_replace_all(self):
    regex_replace_all = lambda pattern, content, replacement: db.execute("select regex_replace_all(?, ?, ?)", [pattern, content, replacement]).fetchone()[0]

    self.assertEqual(
      regex_replace_all('a', 'abc abc', ''),
      'bc bc'
    )

  def test_regex_captures(self):
    MOVIE_PATTERN = "'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)"
    EXAMPLE1 = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931)."
    self.assertEqual(
      execute_all(
        "select rowid, * from regex_captures(?, ?)",
        [MOVIE_PATTERN, EXAMPLE1]
      ),
      [
        {'rowid': 0, 'captures': None},
        {'rowid': 1, 'captures': None},
        {'rowid': 2, 'captures': None},
      ]
    )
    self.assertEqual(
      execute_all(
        """select
          rowid,
          regex_capture(captures, 0) as c0,
          regex_capture(captures, 1) as c1,
          regex_capture(captures, 2) as c2,
          regex_capture(captures, 3) as c3,
          regex_capture(captures, 'title') as title,
          regex_capture(captures, 'year') as year,
          regex_capture(captures, 'not_exist') as not_exist
          from regex_captures(?, ?)
        """,
        [MOVIE_PATTERN, EXAMPLE1]
      ),
      [
        {'rowid': 0, 'c0': '\'Citizen Kane\' (1941)', 'c1': 'Citizen Kane', 'c2': '1941', 'c3': None, 'title': 'Citizen Kane', 'year': '1941', 'not_exist': None, },
        {'rowid': 1, 'c0': '\'The Wizard of Oz\' (1939)', 'c1': 'The Wizard of Oz', 'c2': '1939', 'c3': None, 'title': 'The Wizard of Oz', 'year': '1939', 'not_exist': None, },
        {'rowid': 2, 'c0': '\'M\' (1931)', 'c1': 'M', 'c2': '1931', 'c3': None, 'title': 'M', 'year': '1931', 'not_exist': None, },
      ]
    )

    execute_all("""
        create temp table comments as
        select
          key as rowid,
          value as comment
        from json_each(?)
      """,
      ['["\'Citizen Kane\' (1941), \'The Wizard of Oz\' (1939), \'M\' (1931)", "\'Moonlight\' (2016), \'Arrival\' (2016)", "\'Parasite\' (2020), \'Joker\' (2019), and \'Marriage Story\' (2019)."]']
    )
    self.assertEqual(
      execute_all(
        """
          select
          comments.rowid as comment,
          captures.rowid as capture_idx,
          regex_capture(captures, 'title')  as title,
          regex_capture(captures, 'year')   as year
        from comments
        join regex_captures(
          regex(?),
          comments.comment
        )as captures;
        """, [MOVIE_PATTERN]
      ),
      [
        {'comment': 0, 'capture_idx': 0, 'title': 'Citizen Kane', 'year': '1941'},
        {'comment': 0, 'capture_idx': 1, 'title': 'The Wizard of Oz', 'year': '1939'},
        {'comment': 0, 'capture_idx': 2, 'title': 'M', 'year': '1931'},
        {'comment': 1, 'capture_idx': 0, 'title': 'Moonlight', 'year': '2016'},
        {'comment': 1, 'capture_idx': 1, 'title': 'Arrival', 'year': '2016'},
        {'comment': 2, 'capture_idx': 0, 'title': 'Parasite', 'year': '2020'},
        {'comment': 2, 'capture_idx': 1, 'title': 'Joker', 'year': '2019'},
        {'comment': 2, 'capture_idx': 2, 'title': 'Marriage Story', 'year': '2019'}
      ]
    )
    # with ->> syntax
    if sqlite3.sqlite_version_info[1] >= 38:
      self.assertEqual(
        execute_all(
          """
            select
            comments.rowid as comment,
            captures.rowid as capture_idx,
            captures ->> 'title'  as title2,
            captures ->> 'year'   as year2
          from comments
          join regex_captures(
            regex(?),
            comments.comment
          )as captures;
          """, [MOVIE_PATTERN]
        ),
        [
          {'comment': 0, 'capture_idx': 0, 'title2': 'Citizen Kane', 'year2': '1941'},
          {'comment': 0, 'capture_idx': 1, 'title2': 'The Wizard of Oz', 'year2': '1939'},
          {'comment': 0, 'capture_idx': 2, 'title2': 'M', 'year2': '1931'},
          {'comment': 1, 'capture_idx': 0, 'title2': 'Moonlight', 'year2': '2016'},
          {'comment': 1, 'capture_idx': 1, 'title2': 'Arrival', 'year2': '2016'},
          {'comment': 2, 'capture_idx': 0, 'title2': 'Parasite', 'year2': '2020'},
          {'comment': 2, 'capture_idx': 1, 'title2': 'Joker', 'year2': '2019'},
          {'comment': 2, 'capture_idx': 2, 'title2': 'Marriage Story', 'year2': '2019'}
        ]
      )

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
    self.assertEqual(
      execute_all("""
        with inputs as (
          select value as text
          from json_each(?)
        )
        select matches.rowid, matches.*
        from inputs
        join regex_find_all(regex(?), inputs.text) as matches
        """, ['["Retroactively relinquishing remunerations is reprehensible.", "embezzlements objectivizing"]', '\\b\w{13}\\b']),
      [
        {'rowid': 0, 'start': 0, 'end': 13, 'match': 'Retroactively',},
        {'rowid': 1, 'start': 14, 'end': 27, 'match': 'relinquishing',},
        {'rowid': 2, 'start': 28, 'end': 41, 'match': 'remunerations',},
        {'rowid': 3, 'start': 45, 'end': 58, 'match': 'reprehensible',},
        {'rowid': 0, 'start': 0, 'end': 13, 'match': 'embezzlements',},
        {'rowid': 1, 'start': 14, 'end': 27, 'match': 'objectivizing',},
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
    self.assertEqual(
      execute_all("select rowid, * from regex_split(regex(?), ?)", ['[ \t]+', 'a b \t  c\td    e']),
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
