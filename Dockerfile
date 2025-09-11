# --- Build stage ---
FROM rust:1.86-slim AS builder

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

# --- Runtime stage ---
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && apt-get install -y libpq-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/db-iris-wrapper /usr/local/bin/db-iris-wrapper

ENV ROCKET_CONFIG=/etc/rocket/Rocket.toml
COPY Rocket.toml /etc/rocket/Rocket.toml

EXPOSE 8000

CMD ["db-iris-wrapper"]
