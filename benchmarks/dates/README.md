## Data source

https://github.com/mariomka/regex-benchmark/blob/master/input-text.txt

```
.load lines0

create table corpus as
  select line
  from lines_read('./input-text.txt');
```
