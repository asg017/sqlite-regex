#!/bin/bash
  hyperfine --warmup 10 \
  'sqlite3x words.db ".load ../target/release/libregex0" "select count(*) from words where regexp( regex(\"^[aeiou].*[aeiou]$\"), word);"' \
  'sqlite3x words.db ".load ../target/release/libregex0" "select count(*) from words where regexp( \"^[aeiou].*[aeiou]$\", word);"'