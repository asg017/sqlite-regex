#!/bin/bash
#'sqlite3x benchmarks/test.sqlite ".read benchmarks/this.sql"' \
  
  hyperfine --warmup 10 \
  'sqlite3x benchmarks/test.sqlite ".read benchmarks/this-pointer.sql"' \
  'sqlite3x benchmarks/test.sqlite ".read benchmarks/regexp.sql"' \
  'sqlite3x benchmarks/test.sqlite ".read benchmarks/sqlean.sql"'
#'duckdb.0.5.1 benchmarks/test.duckdb ".read benchmarks/duckdb.sql"' \
#'sqlite3x benchmarks/test.sqlite ".read benchmarks/thisx.sql"'
  