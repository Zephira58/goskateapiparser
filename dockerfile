FROM rust:1.76-slim-bookworm

WORKDIR /app

COPY Cargo.toml ./
COPY Cargo.lock ./

RUN mkdir src/ \
    && echo "fn main() {println!(\"Hello\");}" > src/main.rs \
    && cargo build --release \
    && rm -rf src/target

COPY src/ ./src/

RUN mkdir -p src/data/
COPY go skate. - text channels - trades [1381003312866001037].csv src/data/tradexport_1755362248.csv

RUN cargo build --release

CMD ["./target/release/goskateapi"]
