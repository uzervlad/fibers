-- Add migration script here
create table users (
	id integer primary key,
	username text not null,
	joined_at datetime not null
);
