.load ./regexp

select 
  sum(
    regexp('^([0-9])([0-9])([0-9])([0-9])-([0-9])([0-9])-([0-9])([0-9])$', date)
  ) as total
from dates;
