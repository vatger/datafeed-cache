FROM rust:1.85-bookworm AS builder

WORKDIR /tmp/dfcache
COPY . .

RUN cargo build -p datafeed-cache-server --release

FROM debian:bookworm-20210816-slim
RUN apt update && apt install -y openssl ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /tmp/dfcache/target/release/datafeed_cache_server /usr/local/bin/df-cache

ARG COM_SHA="N/A"
ENV COMMIT_SHA=${COM_SHA}
ENV RUST_LOG=info

EXPOSE 8000

CMD ["df-cache"]

