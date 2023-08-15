#!/usr/bin/env bash
set -x
set -eo pipefail
source "$(dirname $0)/dev.env.sh"

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "  cargo install sqlx-cli --no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

function psql() {
  docker run --rm --network=host -e PGPASSWORD=${DB_PASSWORD} svenmarcus/psql-client $*
}

if [[ -z "${SKIP_DOCKER}" ]]
then
  # Launch postgres using Docker
  docker run \
    --rm \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
  # ^ Increased maximum number of connections for testing purposes
fi

until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  echo >&2 "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} -running migrations now"

sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
