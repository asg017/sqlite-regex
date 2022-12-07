#!/bin/bash
#'sqlite3x benchmarks/test.sqlite ".read benchmarks/this.sql"' \
  
  hyperfine --warmup 10 \
  'sqlite3x test.sqlite ".read this-pointer.sql"' \
  'sqlite3x test.sqlite ".read regexp.sql"' \
  'sqlite3x test.sqlite ".read sqlean.sql"'
#'duckdb.0.5.1 benchmarks/test.duckdb ".read benchmarks/duckdb.sql"' \
#'sqlite3x benchmarks/test.sqlite ".read benchmarks/thisx.sql"'
  
