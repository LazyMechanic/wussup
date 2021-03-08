-- +goose Up
create table if not exists builds
(
    name varchar(128) not null
        constraint builds_pk
        primary key
);

create unique index if not exists builds_name_uindex
    on builds (name);

-- +goose Down
drop table if exists builds;
drop index if exists builds_name_uindex;