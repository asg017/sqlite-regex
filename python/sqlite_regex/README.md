# The `sqlite-regex` Python package

`sqlite-regex` is also distributed on PyPi as a Python package, for use in Python applications. It works well with the builtin [`sqlite3`](https://docs.python.org/3/library/sqlite3.html) Python module.

```
pip install sqlite-regex
```

## Usage

The `sqlite-regex` python package exports two functions: `loadable_path()`, which returns the full path to the loadable extension, and `load(conn)`, which loads the `sqlite-regex` extension into the given [sqlite3 Connection object](https://docs.python.org/3/library/sqlite3.html#connection-objects).

```python
import sqlite_regex
print(sqlite_regex.loadable_path())
# '/.../venv/lib/python3.9/site-packages/sqlite_regex/regex0'

import sqlite3
conn = sqlite3.connect(':memory:')
sqlite_regex.load(conn)
conn.execute('select regex_version(), regex()').fetchone()
# ('v0.1.0', '01gr7gwc5aq22ycea6j8kxq4s9')
```

See [the full API Reference](#api-reference) for the Python API, and [`docs.md`](../../docs.md) for documentation on the `sqlite-regex` SQL API.

See [`datasette-sqlite-regex`](../datasette_sqlite_regex/) for a Datasette plugin that is a light wrapper around the `sqlite-regex` Python package.

## Compatibility

Currently the `sqlite-regex` Python package is only distributed on PyPi as pre-build wheels, it's not possible to install from the source distribution. This is because the underlying `sqlite-regex` extension requires a lot of build dependencies like `make`, `cc`, and `cargo`.

If you get a `unsupported platform` error when pip installing `sqlite-regex`, you'll have to build the `sqlite-regex` manually and load in the dynamic library manually.

## API Reference

<h3 name="loadable_path"><code>loadable_path()</code></h3>

Returns the full path to the locally-install `sqlite-regex` extension, without the filename.

This can be directly passed to [`sqlite3.Connection.load_extension()`](https://docs.python.org/3/library/sqlite3.html#sqlite3.Connection.load_extension), but the [`sqlite_regex.load()`](#load) function is preferred.

```python
import sqlite_regex
print(sqlite_regex.loadable_path())
# '/.../venv/lib/python3.9/site-packages/sqlite_regex/regex0'
```

> Note: this extension path doesn't include the file extension (`.dylib`, `.so`, `.dll`). This is because [SQLite will infer the correct extension](https://www.sqlite.org/loadext.html#loading_an_extension).

<h3 name="load"><code>load(connection)</code></h3>

Loads the `sqlite-regex` extension on the given [`sqlite3.Connection`](https://docs.python.org/3/library/sqlite3.html#sqlite3.Connection) object, calling [`Connection.load_extension()`](https://docs.python.org/3/library/sqlite3.html#sqlite3.Connection.load_extension).

```python
import sqlite_regex
import sqlite3
conn = sqlite3.connect(':memory:')

conn.enable_load_extension(True)
sqlite_regex.load(conn)
conn.enable_load_extension(False)

conn.execute('select regex_version(), regex()').fetchone()
# ('v0.1.0', '01gr7gwc5aq22ycea6j8kxq4s9')
TODO
```
