-- +goose Up
create table settings
(
    platform_id uuid not null
        constraint settings_platforms__fk
            references platforms,
    build_id uuid not null
        constraint settings_builds__fk
            references builds,
    released_ver varchar(64) not null,
    testing_ver varchar(64) not null,
    file_path varchar(256) not null
);

-- +goose Down
drop table if exists settings;