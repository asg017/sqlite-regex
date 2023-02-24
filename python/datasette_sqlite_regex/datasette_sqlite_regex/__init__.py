from datasette import hookimpl
import sqlite_regex

from datasette_sqlite_regex.version import __version_info__, __version__ 

@hookimpl
def prepare_connection(conn):
    conn.enable_load_extension(True)
    sqlite_regex.load(conn)
    conn.enable_load_extension(False)