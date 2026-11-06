#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

case "${1-}" in
	"")
		exec just ash
		;;
	*)
		printf 'Unknown Ash argument: %s\n' "$1" >&2
		exit 2
		;;
esac
