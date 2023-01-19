#!/bin/bash
sqlite3 test.db '.load ../../dist/release/regex0' \
  'select count(*) from corpus where line regexp "[\w]+://[^/\s?#]+[^\s?#]+(?:\?[^\s#]*)?(?:#[^\s]*)?"'