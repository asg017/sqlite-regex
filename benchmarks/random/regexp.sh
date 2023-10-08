#!/bin/bash
sqlite3 test.db '.load ../regexp' \
  'select count(*) from strings where val regexp "1[3-4]";'