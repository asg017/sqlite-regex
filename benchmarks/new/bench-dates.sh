#!/bin/bash
hyperfine --warmup 10 \
  './sqlite-regex-dates.sh' \
  './regexp-dates.sh' \
  './sqlean-re-dates.sh'