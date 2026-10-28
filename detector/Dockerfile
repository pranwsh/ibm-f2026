FROM rust:1.85-slim AS build
WORKDIR /src
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release
FROM debian:bookworm-slim
COPY --from=build /src/target/release/detector /app/detector
EXPOSE 8081
CMD ["/app/detector"]
