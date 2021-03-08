-- +goose Up
create table if not exists tokens
(
    refresh_token uuid not null
        constraint platforms_pk
        primary key,
    client_id uuid not null,
    fingerprint text not null,
    refresh_token_exp int8 not null
);

create unique index if not exists tokens_refresh_token_uindex
    on tokens (refresh_token);

-- +goose Down
drop table if exists tokens;