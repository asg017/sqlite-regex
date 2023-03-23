# sqlite-regex Documentation

A full reference to every function and module that sqlite-regex offers.

As a reminder, sqlite-regex follows semver and is pre v1, so breaking changes are to be expected.

## API Reference

<h3 name="regexp"><code>regexp()</code></h3>

An implementation of the `REGEXP()` operator for SQLite, described here:

> _The REGEXP operator is a special syntax for the regexp() user function. No regexp() user function is defined by default and so use of the REGEXP operator will normally result in an error message. If an application-defined SQL function named "regexp" is added at run-time, then the "X REGEXP Y" operator will be implemented as a call to "regexp(Y,X)"._ >

<small><i><a href="https://www.sqlite.org/lang_expr.html">SQL Language Expressions</a></i>, on sqlite.org</small>

This can be used with the `text regexp pattern` or `regexp(pattern, text)` syntax. See the [regex crate documentation](https://docs.rs/regex/latest/regex/struct.Regex.html) for allowed syntax/features in the regex pattern string.

```sql
select regexp('[abc]', 'a'); -- 1
select regexp('[abc]', 'x'); -- 0

select 'a' regexp '[abc]'; -- 1
select 'x' regexp '[abc]'; -- 0


--
```

<h3 name="regex"><code>regex(pattern)</code></h3>

Creates a regex "object" with the given pattern, using [SQLite's pointer passing interface](https://www.sqlite.org/bindptr.html). Useful when caching regex patterns in heavy queries that use `sqlite-regex` table functions, like [`regex_split()`](#regex_split) or [`regex_find_all()`](#regex_find_all).

Note that the return value will appear to be `NULL` because of SQLite pointer passing interface. To debug, use [`regex_print()`](#regex_print) to print the pattern string of a regex object.

```sql
select regex('[abc]'); -- NULL, but is still a regex "object"
select regex("[abc"); -- Errors with 'Error parsing pattern as regex: ...'

select regex_print(regex('[abc]')); -- '[abc]'
```

<h3 name="regex_print"><code>regex_print(regex)</code></h3>

Prints the pattern of a regex object created with [`regex()`](#regex).

```sql
select regex_print(regex('[abc]')); -- '[abc]'
--
```

<h3 name="regex_valid"><code>regex_valid(pattern)</code></h3>

Returns 1 if the given pattern is a valid regular expression, 0 otherwise.

```sql
select regex_valid('abc'); -- 1
select regex_valid('[abc]'); -- 1
select regex_valid('[abc'); -- 0
select regex_valid(''); -- 1
--
```

<h3 name="regex_find"><code>regex_find(pattern, text)</code></h3>

Find and return the text of the given pattern in the string, or NULL otherwise. Errors if `pattern` is not legal regex. Based on [`Regex.find()`](https://docs.rs/regex/latest/regex/struct.Regex.html#method.find).

```sql
select regex_find(
  '[0-9]{3}-[0-9]{3}-[0-9]{4}',
  'phone: 111-222-3333'
);
-- '111-222-3333'
```

<h3 name="regex_find_all"><code>select * from regex_find_all(pattern, text)</code></h3>

Find all instances of a pattern in the given text. Based on [`Regex.find_iter()`](https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter).

The returned columns:

- `rowid`: The 0-based index of the match.
- `start`: The 0-based index of the starting character of the match inside the text.
- `end`: The 0-based index of the ending character of the match inside the text.
- `match`: The full string match.

For faster results, wrap the pattern with the [`regex()`](#regex) function for caching.

```sql
select rowid, *
from regex_find_all(
  regex('\b\w{13}\b'),
  'Retroactively relinquishing remunerations is reprehensible.'
);
/*
┌───────┬───────┬─────┬───────────────┐
│ rowid │ start │ end │     match     │
├───────┼───────┼─────┼───────────────┤
│ 0     │ 0     │ 13  │ Retroactively │
│ 1     │ 14    │ 27  │ relinquishing │
│ 2     │ 28    │ 41  │ remunerations │
│ 3     │ 45    │ 58  │ reprehensible │
└───────┴───────┴─────┴───────────────┘
```

<h3 name="regex_capture"><code>regex_capture(pattern, text, group)</code></h3>

Returns the text of the capture group with the specific `group` index or name, or NULL otherwise. Errors if `pattern` is not legal regex. Based on [`Regex.captures()`](https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures).

If `group` is a number, then the N-th capture group is returned, where `0` refers to the entire match, `1` refers to the first left-most capture group in the match, `2` the second, and so on. If the provided group number "overflows', then NULL is returned.

```sql
select regex_capture(
  "'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)",
  "Not my favorite movie: 'Citizen Kane' (1941).",
  0
);
-- "'Citizen Kane' (1941)"

select regex_capture(
  "'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)",
  "Not my favorite movie: 'Citizen Kane' (1941).",
  1
);
-- "Citizen Kane"


select regex_capture(
  "'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)",
  "Not my favorite movie: 'Citizen Kane' (1941).",
  2
);
-- "1941"

select regex_capture(
  "'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)",
  "Not my favorite movie: 'Citizen Kane' (1941).",
  3
);
-- NULL
```

If group is a string, then the value of the capture group with the same name is returned. If there is no matching capture group with the name, or the group was not captured, then NULL is returned.

```sql
select regex_capture(
  "'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)",
  "Not my favorite movie: 'Citizen Kane' (1941).",
  'title'
);
-- "Citizen Kane"


select regex_capture(
  "'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)",
  "Not my favorite movie: 'Citizen Kane' (1941).",
  'year'
);
-- "1941"

select regex_capture(
  "'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)",
  "Not my favorite movie: 'Citizen Kane' (1941).",
  'not_exist'
);
-- NULL
```

Note that there is a version of `regex_capture()` that only have two parameters: `captures` and `group`. This can only be used with the [`regex_captures`](#regex_captures) table function, with the special `captures` column like so:

```sql
select
  regex_capture(captures, 'title')      as title,
  regex_capture(captures, 'year')       as year,
  regex_capture(captures, 'not_exist')  as not_exist
from regex_captures(
  regex("'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)"),
  "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931)."
);
/*
┌──────────────────┬──────┬───────────┐
│      title       │ year │ not_exist │
├──────────────────┼──────┼───────────┤
│ Citizen Kane     │ 1941 │           │
│ The Wizard of Oz │ 1939 │           │
│ M                │ 1931 │           │
└──────────────────┴──────┴───────────┘
*/
```

<h3 name="regex_captures"><code>select * from regex_captures(pattern, text)</code></h3>

Returns all non-overlapping capture groups in the given text. Similar to [`regex_find_all`](#regex_find_all), but allows for extracting capture information. Must use with the [`regex_capture`](#regex_capture) function to extract capture group values. Based on [`Regex.captures_iter()`](https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures_iter).

The returned columns:

- `rowid`: The 0-based index of the match. `0` is the entire match, `1` the first matching capture group, `2` the second, etc.
- `captures`: A special value that's meant to be passed into [`regex_capture()`](#regex_capture). Will appear NULL through direct access.

For faster results, wrap the pattern with the [`regex()`](#regex) function for caching.

```sql
select
  rowid,
  captures,
  regex_capture(captures, 0)        as "0",
  regex_capture(captures, 1)        as "1",
  regex_capture(captures, 2)        as "2",
  regex_capture(captures, 3)        as "3"
from regex_captures(
  regex("'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)"),
  "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931)."
);
/*
┌───────┬──────────┬───────────────────────────┬──────────────────┬──────┬───┐
│ rowid │ captures │             0             │        1         │  2   │ 3 │
├───────┼──────────┼───────────────────────────┼──────────────────┼──────┼───┤
│ 0     │          │ 'Citizen Kane' (1941)     │ Citizen Kane     │ 1941 │   │
│ 1     │          │ 'The Wizard of Oz' (1939) │ The Wizard of Oz │ 1939 │   │
│ 2     │          │ 'M' (1931)                │ M                │ 1931 │   │
└───────┴──────────┴───────────────────────────┴──────────────────┴──────┴───┘
*/
```

```sql
select
  rowid,
  captures,
  regex_capture(captures, 'title')  as title,
  regex_capture(captures, 'year')  as year,
  regex_capture(captures, 'blah')  as blah
from regex_captures(
  regex("'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)"),
  "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931)."
);
/*
┌───────┬──────────┬──────────────────┬──────┬──────┐
│ rowid │ captures │      title       │ year │ blah │
├───────┼──────────┼──────────────────┼──────┼──────┤
│ 0     │          │ Citizen Kane     │ 1941 │      │
│ 1     │          │ The Wizard of Oz │ 1939 │      │
│ 2     │          │ M                │ 1931 │      │
└───────┴──────────┴──────────────────┴──────┴──────┘
*/
```

<h3 name="regex_replace"><code>regex_replace(pattern, text, replacement)</code></h3>

Replace the **first** instance of `pattern` inside `text` with the given `replacement` text. Supports the [replacment string syntax](https://docs.rs/regex/latest/regex/struct.Regex.html#replacement-string-syntax). Based on [`Regex.replace()`](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)

```sql

select regex_replace(
  '[^01]+',
  '1078910',
  ''
);
-- '1010'

select regex_replace(
  '(?P<last>[^,\s]+),\s+(?P<first>\S+)',
  'Springsteen, Bruce',
  '$first $last'
);
-- 'Bruce Springsteen'
```

<h3 name="regex_replace_all"><code>regex_replace_all(pattern, text, replacement)</code></h3>

Replace **all** instance of `pattern` inside `text` with the given `replacement` text. Supports the [replacment string syntax](https://docs.rs/regex/latest/regex/struct.Regex.html#replacement-string-syntax). Based on [`Regex.replace_all()`](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace_all)

```sql

select regex_replace_all(
  'dog',
  'cat dog mouse dog',
  'monkey'
)
-- 'cat monkey mouse monkey'
```

<h3 name="regex_split"><code>select * from regex_split(pattern, text)</code></h3>

Split the given text on each instance of the given pattern. Based on [`Regex.split()`](https://docs.rs/regex/latest/regex/struct.Regex.html#method.split).

The returned columns:

- `rowid`: The 0-based index of the split item.
- `item`: The individual split item, as text.

For faster results, wrap the pattern with the [`regex()`](#regex) function for caching.

```sql
select rowid, *
from regex_split(
  regex('[ \\t]+'),
  'a b \t  c\td    e'
);
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

<h3 name="regexset"><code>regexset(pattern1, patern2, ...)</code></h3>

Creates a regexset "object" with the given pattern, using [SQLite's pointer passing interface](https://www.sqlite.org/bindptr.html). Required when using `regexset_is_match` and `regexset_matches`. Based on [`RegexSet`](https://docs.rs/regex/latest/regex/struct.RegexSet.html).

Note that the return value will appear to be `NULL` because of SQLite pointer passing interface. To debug, use [`regexset_print()`](#regexset_print) to print the pattern string of a regex object.

```sql
select regexset(
  "bar",
  "foo",
  "barfoo"
);
-- NULL, but is still a regexset "object"

select regexset("[abc"); --errors

select regexset_print(regexset('abc', 'xyz')); -- '["abc","xyz"]'
```

<h3 name="regexset_print"><code>regexset_print()</code></h3>

Prints the patterns of a regexset object created with [`regexset()`](#regexset).

```sql
select regexset_print(regexset('abc', 'xyz')); -- '["abc","xyz"]'
```

<h3 name="regexset_is_match"><code>regexset_is_match(regexset, text)</code></h3>

Returns 1 if any of the patterns in `regexset` matches `text`. Based on [`RegexSet.is_match()`](https://docs.rs/regex/latest/regex/struct.RegexSet.html#method.is_match).

```sql
select regexset_is_match(
  regexset(
    "bar",
    "foo",
    "barfoo"
  ),
  'foobar'
); -- 1

select regexset_is_match(
  regexset(
    "bar",
    "foo",
    "barfoo"
  ),
  'xxx'
); -- 0

```

<h3 name="regexset_matches"><code>select * from regexset_matches(regexset, text)</code></h3>

Returns all the matching patterns inside `regexset` found inside `text`. Note that this doesn't return rows for each of the matches themselves, only if there was at least 1 match for each patten. Based on [`RegexSet.matches()`](https://docs.rs/regex/latest/regex/struct.RegexSet.html#method.matches).

```sql
select
  key,
  pattern
from regexset_matches(
  regexset(
    '\w+',
    '\d+',
    '\pL+',
    'foo',
    'bar',
    'barfoo',
    'foobar'
  ),
  'foobar'
);
/*
┌─────┬─────────┐
│ key │ pattern │
├─────┼─────────┤
│ 0   │ \w+     │
│ 2   │ \pL+    │
│ 3   │ foo     │
│ 4   │ bar     │
│ 6   │ foobar  │
└─────┴─────────┘
*/
```

<h3 name="regex_version"><code>regex_version()</code></h3>

Returns the semver version string of the current version of sqlite-regex.

```sql
select regex_version();
-- "v0.1.0"
```

<h3 name="regex_debug"><code>regex_debug()</code></h3>

Returns a debug string of various info about sqlite-regex, including
the version string, build date, and commit hash.

```sql
select regex_debug();
/*
Version: v0.0.0-alpha.4
Source: 85fd18bea80c42782f35975351ea3760d4396eb6
*/
```
