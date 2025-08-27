#!/usr/bin/env bash

set -euo pipefail

client() {
  (cd client && dx serve -p client)
}

tailwind() {
  (cd client && bun i && bun run tailwind)
}

server() {
  (cd server && cargo run -p server --bin server)
}

if [[ $# -eq 0 ]]; then
  echo "No arguments provided. Please specify 'client', 'tailwind', or 'server'."
  exit 1
fi

case $1 in
client)
  client
  ;;
tailwind)
  tailwind
  ;;
server)
  server
  ;;
*)
  echo "Invalid argument. Please specify 'client', 'tailwind', or 'server'."
  exit 1
  ;;
esac
