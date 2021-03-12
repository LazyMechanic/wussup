-- +goose Up
create table if not exists files
(
    id uuid not null
        constraint files_pk
        primary key,
    platform varchar(128) not null
        constraint files_platforms__fk
        references platforms,
    build varchar(128) not null
        constraint files_builds__fk
        references builds,
    version varchar(128) not null
);

create unique index if not exists files_id_uindex
    on files (id);

-- +goose Down
drop table if exists files;
drop index if exists files_id_uindex;