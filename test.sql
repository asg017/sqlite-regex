.load ./dist/debug/regex0

select * 
from regex_split(
  regex('[ \\t]+'), 
  'a b \t  c\td    e'
);