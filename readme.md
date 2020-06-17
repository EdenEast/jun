# Jun - An example Juniper backend

This is an example of creating a graphql backend with [Juniper]. This is a sandbox for me to learn
and experament with the libaray. I have two sample implemenation for [Tide] and [Actix].

[juniper]: https://github.com/graphql-rust/juniper
[tide]: https://github.com/http-rs/tide
[actix]: https://github.com/actix/actix-web

## Quick Start

### Development Dependencies

`Jun` uses [Just] as an alternative to makefile. It also uses the currently pre-released [sqlx-cli].
To install:

```bash
cargo install just
cargo install --git https://github.com/launchbadge/sqlx sqlx-cli
```

`Jun` also uses [Docker] and [Docker Compose][docker-compose] to spin up a development [postgres] server.
Install them on your system. If you are using Windows, docker for windows now can use WSL2 as a it's
engine instead of a virtual machine. More information about this backend [here][docker-wsl2]

[just]: https://github.com/casey/just
[sqlx-cl]: https://github.com/launchbadge/sqlx/tree/master/sqlx-cli
[docker]: https://www.docker.com/
[docker-compose]: https://docs.docker.com/compose/
[postgres]: https://www.postgresql.org/
[docker-wsl2]: https://docs.docker.com/docker-for-windows/wsl/

### Development Setup

In the just file there is a recipe called `up` that will being up the postgres database and
provision it with all of the migrations. To setup a development environment just execute this
recipe.

```bash
# start pg server and execute migrations
just up
```

If you did not want to use just then you can manually setup.

```bash
# start pg server
docker-compose up -d postgres

# copy the example env to .env file
cp .env.example .env

# execute sqlx to run migrations to database
sqlx migrate run
```

Executing an example backend.

```bash
# run the actix integration
cargo run --bin jun-actix

# run the tide integration
cargo run --bin jun-tide
```

