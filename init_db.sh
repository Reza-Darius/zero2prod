#!/usr/bin/env bash
# enables shell tracing
set -x
# Exit the script immediately if any command returns a non-zero exit status.
set -eo pipefail

source .env

CONTAINER_NAME="zero2prod"
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${DB_NAME:=database}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

# Capture the container ID so we can exec into it later
CONTAINER_ID=$(docker run \
  --tmpfs /var/lib/postgresql \
  --name "${CONTAINER_NAME}" \
  -e POSTGRES_USER="${DB_USER}" \
  -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
  -e POSTGRES_DB="${DB_NAME}" \
  -p "${DB_PORT}":5432 \
  -d postgres:18 \
  postgres -N 1000)

# Poll pg_isready *inside* the container, no local psql/pg_isready needed
until docker exec "${CONTAINER_ID}" pg_isready -U "${DB_USER}" > /dev/null 2>&1; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
