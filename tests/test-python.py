import unittest
import sqlite3
import sqlite_regex

class TestSqliteregexPython(unittest.TestCase):
  def test_path(self):
    db = sqlite3.connect(':memory:')
    db.enable_load_extension(True)

    self.assertEqual(type(sqlite_regex.loadable_path()), str)
    
    sqlite_regex.load(db)
    version, result = db.execute('select regex_version(), regexp("[abc]", "c")').fetchone()
    self.assertEqual(version[0], "v")
    self.assertEqual(result, 1)

if __name__ == '__main__':
    unittest.main()