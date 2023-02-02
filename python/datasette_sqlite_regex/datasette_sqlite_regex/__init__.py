from datasette import hookimpl
import sqlite_regex

@hookimpl
def prepare_connection(conn):
    conn.enable_load_extension(True)
    sqlite_regex.load(conn)
    conn.enable_load_extension(False)