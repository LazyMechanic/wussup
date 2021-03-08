-- +goose Up
create table if not exists platforms
(
    id   uuid         not null
    constraint platforms_pk
    primary key,
    name varchar(128) not null
    );

create unique index if not exists platforms_id_uindex
    on platforms (id);

create unique index if not exists platforms_name_uindex
    on platforms (name);

-- +goose Down
drop table if exists platforms;
drop index if exists platforms_id_uindex;
drop index if exists platforms_name_uindex;