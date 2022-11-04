gcc regexp.c -fPIC -shared -o regexp.dylib -I /Users/alex/projects/sqlite-lines/sqlite

## TODO

- [ ] `regex_valid(pattern)`
- [ ] `regex_find(pattern, content)`

- [ ] `regex_replace(pattern, content)`
- [ ] `regex_replace_all(pattern, content)`
- [ ] `regex_replace_n(pattern, content, n)`

- [ ] `regex_shortest_match(pattern, content)`
- [ ] `regex_shortest_match_at(pattern, content, after)`
- [ ] `regex_shortest_is_match_at(pattern, content, offset)`

- [ ] `select * from regex_captures(pattern, text)`
- [ ] `select value from regex_split(pattern, text)`
- [ ] `regex_`
- [ ] `regex_`

0 vtabs

- iter
- capture_groups
- split
- capture names (given regex)

benchmarks

- preoload data into tables
- mix matching and not matching
