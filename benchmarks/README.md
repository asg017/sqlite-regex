# `sqlite-regex` Benchmarks

## Caveat: Benchmarks are hard and easy to game

This benchmark isn't exhaustive, and only benchmarks between other widely-used SQLite regex extensions.

## `REGEXP()` across all SQLite regex extensions

![](./dates.png)

Explaination: Essentially running `select count(*) from corpus where line regexp "\d{4}-\d{2}-\d{2}"`, though `regexp` and `sqlean/re` doesn't support `\d` or `{4}` syntax.

```
gcc -O3 -shared -fPIC regexp.c -o regexp.dylib

gcc -O3 -shared -fPIC -I./ re.c sqlite3-re.c -o re.dylib
```
