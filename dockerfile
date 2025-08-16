FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release
COPY src/ ./src/
RUN rm -f target/release/deps/goskateapiparser* && \
    cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY src/data ./data
COPY --from=builder /usr/src/app/target/release/goskateapiparser .
RUN groupadd --gid 1000 appuser && \
    useradd --uid 1000 --gid 1000 -m appuser && \
    chown -R appuser:appuser /app
USER appuser
CMD ["./goskateapiparser"]
