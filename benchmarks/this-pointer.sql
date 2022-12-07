.load ../dist/release/regex0

select sum(
  regexp(regex('^\d{4}-\d{2}-\d{2}$'), date)
)
from dates;
