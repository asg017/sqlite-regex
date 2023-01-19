#!/bin/bash
sqlite3 test.db '.load ../re' \
  'select count(*) from corpus where line regexp "([0-9])([0-9])([0-9])([0-9])-([0-9])([0-9])-([0-9])([0-9])"'