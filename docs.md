# sqlite-regex Documentation

A full reference to every function and module that sqlite-regex offers.

As a reminder, sqlite-regex follows semver and is pre v1, so breaking changes are to be expected.

## API Reference

<h3 name="regexp"><code>regexp()</code></h3>

```sql
select regexp('[abc]', 'a'); -- 1
select regexp('[abc]', 'x'); -- 0

select 'a' regexp '[abc]'; -- 1
select 'x' regexp '[abc]'; -- 0


--
```

> _The REGEXP operator is a special syntax for the regexp() user function. No regexp() user function is defined by default and so use of the REGEXP operator will normally result in an error message. If an application-defined SQL function named "regexp" is added at run-time, then the "X REGEXP Y" operator will be implemented as a call to "regexp(Y,X)"._ >

<small><i><a href="https://www.sqlite.org/lang_expr.html">SQL Language Expressions</a></i>, on sqlite.org</small>

<h3 name="regex"><code>regex(pattern)</code></h3>

Creates a regex object with the given pattern.

https://docs.rs/regex/latest/regex/struct.Regex.html

https://www.sqlite.org/bindptr.html

```sql
select regex("[abc]"); -- NULL
select regex("[abc"); -- Errors with ''
select regex_print(regex())
--
```

<h3 name="regex_print"><code>regex_print(regex)</code></h3>

```sql
select regex_print();
--
```

<h3 name="regex_valid"><code>regex_valid(pattern)</code></h3>

```sql
select regex_valid();
--
```

<h3 name="regex_find"><code>regex_find()</code></h3>

https://docs.rs/regex/latest/regex/struct.Regex.html#method.find

```sql
select regex_find();
--
```

<h3 name="regex_find_all"><code>select * from regex_find_all()</code></h3>

https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter

```sql
select * from regex_find_all();
```

<h3 name="regex_replace"><code>regex_replace()</code></h3>

https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace

```sql
select regex_replace();
--
```

<h3 name="regex_split"><code>select * from regex_split()</code></h3>

https://docs.rs/regex/latest/regex/struct.Regex.html#method.split

```sql
select * from regex_split();
```

<h3 name="regexset"><code>regexset()</code></h3>

https://docs.rs/regex/latest/regex/struct.RegexSet.html

```sql
select regexset();
--
```

<h3 name="regexset_print"><code>regexset_print()</code></h3>

```sql
select regexset_print();
--
```

<h3 name="regexset_is_match"><code>regexset_is_match()</code></h3>
https://docs.rs/regex/latest/regex/struct.Regex.html#method.is_match
https://docs.rs/regex/latest/regex/struct.RegexSet.html#method.is_match

```sql
select regexset_is_match();
--
```

<h3 name="regexset_matches"><code>select * from regexset_matches()</code></h3>

https://docs.rs/regex/latest/regex/struct.RegexSet.html#method.matches

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
```

<h3 name="regex_version"><code>regex_version()</code></h3>

Returns the semver version string of the current version of sqlite-regex.

```sql
select regex_version();
-- "v0.0.1"
```

<h3 name="regex_debug"><code>regex_debug()</code></h3>

Returns a debug string of various info about sqlite-regex, including
the version string, build date, and commit hash.

```sql
select regex_debug();
-- "TODO"
```
