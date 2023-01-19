#!/bin/bash
sqlite3 test.db '.load ../../dist/release/regex0' \
  'select count(*) from corpus where line regexp "(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])"'