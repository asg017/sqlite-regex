#!/bin/bash
hyperfine --warmup 5 \
  'duckdb benchmarks/test.duckdb ".read benchmarks/duckdb.sql"' \
  'sqlite3x benchmarks/test.sqlite ".read benchmarks/this.sql"' \
  'sqlite3x benchmarks/test.sqlite ".read benchmarks/sqlean.sql"' \
  'sqlite3x benchmarks/test.sqlite ".read benchmarks/regexp.sql"'