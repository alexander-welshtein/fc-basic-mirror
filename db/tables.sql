create table if not exists card
(
    id    bigserial primary key,
    front varchar not null,
    back  varchar not null,
    date  timestamptz default now()
);