.load ./target/release/libregex0

select sum(
  regexp('^\d{4}-\d{2}-\d{2}$', date)
)
from dates;