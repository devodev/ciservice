#!/usr/bin/env bash
set -e
set -u

# used to compute db url
DATABASE_USER="${DATABASE_USER:-"postgres"}"
DATABASE_PASSWORD="${DATABASE_PASSWORD:-"postgres"}"
DATABASE_HOST="${DATABASE_HOST:-"localhost"}"
DATABASE_PORT="${DATABASE_PORT:-"5432"}"
DATABASE_NAME="${DATABASE_NAME:-"ciservice"}"
# the name of the rocket db defined in main.rs
ROCKET_DB_NAME="${ROCKET_DB_NAME:-"postgres_db"}"

# used by diesel for setup/migrations
export DATABASE_URL="${DATABASE_URL:-"postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}"}"
# used by rocket to provide db url
export ROCKET_DATABASES="${ROCKET_DATABASES:-"{${ROCKET_DB_NAME}={url=\"${DATABASE_URL}\"}}"}"

# This .env file allows diesel to resolve the running database when using docker exec.
echo "DATABASE_URL=${DATABASE_URL}" > .env

function migrations {
    echo "[entrypoint] waiting for db to be ready or bail out after max retries"
    ( retries=15; while ! nc -z "${DATABASE_HOST}" "${DATABASE_PORT}"; do ((--retries)) || exit 1; sleep 1; done )
    echo "[entrypoint] db ready"

    echo "[entrypoint] running migrations"
    diesel setup
    diesel migration run
}

if [ "$1" = "migrations" ]; then
    migrations
    exit 0
fi

if [ "$1" = "ciservice" ]; then
    migrations

    echo "[entrypoint] starting ciservice"
    exec "$@"
fi

exec "$@"
