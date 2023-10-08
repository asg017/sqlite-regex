#!/bin/bash
sqlite3 test.db '.load ../../dist/release/regex0' \
  'select count(*) from strings where val regexp "1[3-4]";'