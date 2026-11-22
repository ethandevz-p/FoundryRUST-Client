#!/bin/sh
# Starts foundry-server in the server folder of a hosting panel (the Pterodactyl egg; docs/hosting.md).
# The program sits in ./foundry/, the prepared game files in ./resources/. The first start writes server.toml
# from the panel's FOUNDRY_* variables (foundry-server init: FOUNDRY_EULA, WORLD, SEED, WHITELIST, PLAYERS, OP,
# ONLINE); the port always follows the panel's allocation (SERVER_PORT).
cd "$(dirname "$0")" || exit 1
if [ ! -f server.toml ]; then
  ./foundry/foundry-server init --yes --port "${SERVER_PORT:-25565}" || exit 1
fi
sed -i "s/^port = .*/port = ${SERVER_PORT:-25565}/" server.toml
exec ./foundry/foundry-server run
