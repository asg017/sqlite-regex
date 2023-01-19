#!/bin/bash
sqlite3 test.db '.load ../../dist/release/regex0' \
  'select count(*) from corpus where line regexp "\d{4}-\d{2}-\d{2}"'