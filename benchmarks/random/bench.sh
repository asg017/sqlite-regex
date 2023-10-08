#!/bin/bash
hyperfine --warmup 10 --export-json=results.json \
  './sqlite-regex.sh' \
  './regexp.sh' \
  './sqlean-re.sh'
