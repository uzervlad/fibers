-- Add migration script here
create table beatmapsets (
  id integer primary key,
  artist text not null,
  title text not null,
  creator text not null,
  creator_id int not null
);

create table beatmaps (
  id integer primary key,
  beatmapset_id integer,
  version text not null
);
