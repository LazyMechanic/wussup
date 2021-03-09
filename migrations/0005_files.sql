-- +goose Up
create table if not exists files
(
    id uuid not null
        constraint files_pk
        primary key,
    path varchar(4096) not null
);

create unique index if not exists files_id_uindex
    on files (id);

-- +goose Down
drop table if exists files;
drop index if exists files_id_uindex;