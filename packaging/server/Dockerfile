# foundry-server in a container (docs/server.md, section 3). Build context: the repository root:
#   docker build -f packaging/server/Dockerfile -t foundry-server .
#
# The image holds none of Minecraft's files. Prepare them on the host from your own Minecraft 26.3
# (foundry-prepare --prepare, see packaging/PLAY.txt) and mount the folder it writes, read-only:
#   docker run -d --name foundry -p 25565:25565 --stop-timeout 180 \
#     -v foundry-data:/data \
#     -v /path/to/.local/foundry-client/26.3:/opt/foundry/.local/foundry-client/26.3:ro \
#     -e FOUNDRY_EULA=true -e FOUNDRY_WHITELIST=true -e FOUNDRY_PLAYERS=Alex,Steve -e FOUNDRY_OP=Alex \
#     foundry-server
#
# FOUNDRY_EULA=true accepts the Minecraft EULA (https://aka.ms/MinecraftEULA) for this server. The first
# start writes /data/server.toml from FOUNDRY_* (EULA, WORLD, SEED, PORT, WHITELIST, PLAYERS, OP); later
# starts use the file. Manage it with `docker exec foundry foundry-server status|cmd|backup|logs ...`.
FROM rust:1-slim-bookworm AS build
WORKDIR /src
COPY . .
# The server build of the game (no sound, no controllers: it needs neither ALSA nor udev) as foundry-engine.
RUN cargo build --release --locked -p foundry-game --no-default-features --bin foundry-client \
    && cargo build --release --locked -p foundry-server --bin foundry-server \
    && mkdir /out && cp target/release/foundry-client /out/foundry-engine && cp target/release/foundry-server /out/

FROM debian:bookworm-slim
RUN useradd --system --home-dir /data --shell /usr/sbin/nologin foundry \
    && mkdir -p /data /opt/foundry/.local/foundry-client \
    && chown foundry:foundry /data
COPY --from=build /out/ /opt/foundry/
ENV PATH=/opt/foundry:$PATH \
    FOUNDRY_NO_AUDIO=1
WORKDIR /data
VOLUME /data
EXPOSE 25565
USER foundry
# SIGTERM makes foundry-server stop the game cleanly (it saves first): give it --stop-timeout 180.
CMD ["sh", "-c", "test -f server.toml || foundry-server init --yes || exit 1; exec foundry-server run"]
