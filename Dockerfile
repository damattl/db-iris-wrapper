# Build React
FROM node:22-alpine AS react-builder
WORKDIR /app

COPY app/package.json app/yarn.lock ./

RUN yarn install

COPY app/ ./
RUN yarn build

# Build Rust
FROM rust:1.86-slim AS rust-builder

RUN apt-get update \
    && apt-get install -y libpq-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY crates ./crates
RUN cargo fetch
RUN cargo build --all --release

RUN ls -l
RUN ls -l ./target/release

# Runtime
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && apt-get install -y libpq-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=rust-builder /app/target/release/db-iris-wrapper /usr/local/bin/db-iris-wrapper
COPY --from=react-builder /app/dist/ ./static

RUN ls /usr/local/bin/
RUN ls ./

# TODO: Make optional
ENV STATIONS_SRC=SQL:/etc/db-iris-wrapper/stations.sql
COPY stations.sql /etc/db-iris-wrapper/stations.sql

ENV STATUS_CODES_SRC=EXCEL:/etc/db-iris-wrapper/codes.xlsx
COPY codes.xlsx /etc/db-iris-wrapper/codes.xlsx

ENV ROCKET_CONFIG=/etc/rocket/Rocket.toml
COPY Rocket.toml /etc/rocket/Rocket.toml

EXPOSE 8000

CMD ["db-iris-wrapper"]
