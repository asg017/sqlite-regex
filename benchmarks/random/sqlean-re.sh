#!/bin/bash
sqlite3 test.db '.load ../re' \
  'select count(*) from strings where val regexp "1[3-4]";'