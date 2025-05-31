-- Add migration script here
create table scores (
	online_id integer primary key,
	data_json blob not null
);
