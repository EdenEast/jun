mintty_prefix := `if [[ "$(uname -s)" =~ 'MINGW' ]]; then echo 'winpty'; else echo '' ; fi`

_list:
    just --list

# the sleep just gives enough time for the database to be responsive
setup:
    @if [ -z `docker-compose ps -q postgres` ] || [ -z `docker ps -q --no-trunc | grep $(docker-compose ps -q postgres)` ]; then docker-compose up -d postgres ; sleep 1 ; fi
    @rm .env && cp .env.example .env

migrate: setup
    sqlx migrate run

connect: migrate
    {{mintty_prefix}} docker-compose exec postgres psql -h localhost -p 5432 -U june june

