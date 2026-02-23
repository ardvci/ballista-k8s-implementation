FROM rust:1.92-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libssl-dev \
    pkg-config \
    cmake \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/data-gen /usr/local/bin/data-gen
COPY --from=builder /app/target/release/run /usr/local/bin/run

RUN chmod +x /usr/local/bin/data-gen /usr/local/bin/run

# Will be overriden on YAML Files
CMD ["data-gen"]