-- +goose Up
create table if not exists sessions
(
    refresh_token uuid not null
        constraint sessions_pk
        primary key,
    client_id uuid not null,
    fingerprint text not null,
    refresh_token_exp timestamp not null
);

create unique index if not exists sessions_refresh_token_uindex
    on sessions (refresh_token);

-- +goose Down
drop table if exists sessions;
drop index if exists sessions_refresh_token_uindex;