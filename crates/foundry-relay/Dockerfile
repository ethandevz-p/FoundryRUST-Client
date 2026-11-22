# The Play with friends relay, for anyone to run (docs/play-with-friends.md, "Running your own relay").
# Build context: the repository root (the workspace's manifests and lockfile):
#   docker build -f crates/foundry-relay/Dockerfile -t foundry-relay .
FROM rust:1-slim-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release --locked -p foundry-relay \
    && cp target/release/foundry-relay /foundry-relay

FROM debian:bookworm-slim
COPY --from=build /foundry-relay /usr/local/bin/foundry-relay
ENV PORT=7000
EXPOSE 7000
USER nobody
CMD ["foundry-relay"]
