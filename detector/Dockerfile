FROM rust:1.89-slim AS build
RUN apt-get update && apt-get install -y --no-install-recommends build-essential cmake pkg-config libssl-dev zlib1g-dev libcurl4-openssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates wget && rm -rf /var/lib/apt/lists/*
COPY --from=build /src/target/release/detector /app/detector
EXPOSE 8081
CMD ["/app/detector"]
