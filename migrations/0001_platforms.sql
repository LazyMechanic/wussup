-- +goose Up
create table if not exists platforms
(
    name varchar(128) not null
        constraint platforms_pk
        primary key
);

create unique index if not exists platforms_name_uindex
    on platforms (name);

-- +goose Down
drop table if exists platforms;
drop index if exists platforms_name_uindex;