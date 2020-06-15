create extension if not exists "uuid-ossp";

create table if not exists profile (
    id uuid default uuid_generate_v4() primary key,
    username varchar not null unique,
    email varchar not null unique,
    password_hash varchar not null,
    image varchar null,
    created_at timestamp not null default (now() at time zone 'utc'),
    updated_at timestamp not null default (now() at time zone 'utc')
);

create table if not exists auth_token (
    id uuid default uuid_generate_v4() primary key,
    user_id uuid not null,
    token uuid default uuid_generate_v4(),
    created_at timestamp not null default (now() at time zone 'utc'),
    updated_at timestamp not null default (now() at time zone 'utc'),
    foreign key (user_id) references profile(id)
);
