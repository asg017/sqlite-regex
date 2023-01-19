create table dates as 
  select 
    date('now', format('-%d days', value)) as date
  from generate_series(1, 1e6);