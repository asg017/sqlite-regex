create table dates as 
  select 
    ((DATE '1992-03-22') + generate_series::int)::text as t
  from generate_series(1, 1000000)