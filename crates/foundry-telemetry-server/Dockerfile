# Built from the repository root: docker build -f crates/foundry-telemetry-server/Dockerfile .
FROM rust:1-slim AS build
WORKDIR /src
# The root manifest only for the shared crate's workspace settings (edition, lints).
COPY Cargo.toml Cargo.toml
# Every crate the server's Cargo.lock builds from a path (tests/test_service_images.py checks the list).
COPY crates/foundry-addresses crates/foundry-addresses
COPY crates/foundry-bench crates/foundry-bench
COPY crates/foundry-core crates/foundry-core
COPY crates/foundry-guest-stack crates/foundry-guest-stack
COPY crates/foundry-hooks crates/foundry-hooks
COPY crates/foundry-perf-format crates/foundry-perf-format
COPY crates/foundry-platform crates/foundry-platform
COPY crates/foundry-system crates/foundry-system
COPY crates/foundry-telemetry crates/foundry-telemetry
COPY crates/foundry-telemetry-server crates/foundry-telemetry-server
RUN cargo build --release --locked --manifest-path crates/foundry-telemetry-server/Cargo.toml

FROM debian:stable-slim
COPY --from=build /src/crates/foundry-telemetry-server/target/release/foundry-telemetry-server /usr/local/bin/
# Railway's proxy sets X-Real-IP to the client's address; it is the header the server read first before
# FOUNDRY_TELEMETRY_CLIENT_IP_HEADER existed, so this keeps the same address for the rate limits (README "Hosting").
ENV FOUNDRY_TELEMETRY_DB=/data/telemetry.sqlite3 FOUNDRY_TELEMETRY_TRUST_PROXY=1 \
    FOUNDRY_TELEMETRY_CLIENT_IP_HEADER=x-real-ip
CMD ["foundry-telemetry-server"]
