select number from numbers_mt(10) where number > 5  and exists (select name from system.settings);
select number from numbers_mt(10) where number > 5  and exists (select name from system.settings) and exists (select number from numbers_mt(10));
select number from numbers_mt(10) where number > 5  and exists (select name from system.settings where exists (select number from numbers_mt(10)));
select number from numbers_mt(20) where number > 15  and not exists (select number from numbers_mt(5) where number > 10);