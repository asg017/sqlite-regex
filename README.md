# sqlite-regex

A fast and performant SQLite extension for regular expressions.

See [`sqlite-loadable-rs`](https://github.com/asg017/sqlite-loadable-rs), the framework that makes this extension possible.

## WORK IN PROGRESS

This extension isn't 100% complete yet, but hoping to release in the next 1-2 weeks! A sneak peek at what to expect:

### The fastest `REGEXP()` implementation in SQLite

I don't have a fancy benchmark screenshot yet, but in my Mac, I get ~50% faster results with the `regexp()` in `sqlite-regex` over the "official" [regexp.c](https://github.com/sqlite/sqlite/blob/master/ext/misc/regexp.c) SQLite extension.

### More regex utilities

Very rarely does `regexp` cover all your regular expression needs. `sqlite-regex` also includes support for many other regex operations, such as:

**Find all occurances of a pattern in a string**

```sql
select regex_find('[0-9]{3}-[0-9]{3}-[0-9]{4}', 'phone: 111-222-3333');
-- '111-222-3333'

select rowid, *
from regex_find_all('\b\w{13}\b', 'Retroactively relinquishing remunerations is reprehensible.');
/*
┌───────┬───────┬─────┬───────────────┐
│ rowid │ start │ end │     match     │
├───────┼───────┼─────┼───────────────┤
│ 0     │ 0     │ 13  │ Retroactively │
│ 1     │ 14    │ 27  │ relinquishing │
│ 2     │ 28    │ 41  │ remunerations │
│ 3     │ 45    │ 58  │ reprehensible │
└───────┴───────┴─────┴───────────────┘
*/
```

**Split the string on the given pattern delimiter**

```sql
select rowid, *
from regex_split('[ \t]+', 'a b     c d    e');
/*
┌───────┬──────┐
│ rowid │ item │
├───────┼──────┤
│ 0     │ a    │
│ 1     │ b    │
│ 2     │ c    │
│ 3     │ d    │
│ 4     │ e    │
└───────┴──────┘
*/
```

**Replace occurances of a pattern with another string**

```sql
select regex_replace(
  '(?P<last>[^,\s]+),\s+(?P<first>\S+)',
  'Springsteen, Bruce',
  '$first $last'
);
-- 'Bruce Springsteen'

select regex_replace_all('a', 'abc abc', '');
-- 'bc bc'
```

And more!
