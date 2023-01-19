select 
  sum(
    regexp_matches(t, '^([0-9])([0-9])([0-9])([0-9])-([0-9])([0-9])-([0-9])([0-9])$')::int
  ) as matches
from dates;