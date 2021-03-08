-- +goose Up
create table if not exists settings
(
    platform varchar(128) not null
        constraint settings_platforms__fk
            references platforms,
    build varchar(128) not null
        constraint settings_builds__fk
            references builds,
    released_ver varchar(64) not null,
    testing_ver varchar(64) not null,
    file_path varchar(256) not null
);

-- +goose Down
drop table if exists settings;