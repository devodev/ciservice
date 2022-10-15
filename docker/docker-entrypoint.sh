#!/usr/bin/env bash
set -e

if [ "$1" = "ciservice" ]; then
    exec "$@"
fi

exec "$@"
