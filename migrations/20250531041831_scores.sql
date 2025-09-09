-- Add migration script here
create table scores (
	online_id integer primary key,
	beatmap_id integer not null,
	user_id integer not null,
	data_json blob not null
);
