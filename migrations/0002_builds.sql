-- +goose Up
create table if not exists builds
(
    id   uuid         not null
    constraint builds_pk
    primary key,
    name varchar(128) not null
    );

create unique index if not exists builds_id_uindex
    on builds (id);

create unique index if not exists builds_name_uindex
    on builds (name);

-- +goose Down
drop table if exists builds;