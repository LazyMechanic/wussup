-- +goose Up
create table if not exists settings
(
    id uuid not null
        constraint settings_pk
        primary key,
    platform varchar(128) not null
        constraint settings_platforms__fk
            references platforms,
    build varchar(128) not null
        constraint settings_builds__fk
            references builds,
    released_file_id uuid not null
        constraint settings_released_file_ids__fk
            references files,
    testing_file_id uuid not null
        constraint settings_testing_file_ids__fk
            references files
);

create unique index if not exists settings_id_uindex
    on settings (id);

-- +goose Down
drop table if exists settings;
drop index if exists settings_id_uindex;