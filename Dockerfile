FROM rust:1.85-bookworm AS builder

WORKDIR /tmp/dfcache
COPY . .

RUN cargo build --release

FROM debian:bookworm-20210816-slim
RUN apt update && apt install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /tmp/dfcache/target/release/df-cache /usr/local/bin/df-cache

ARG COM_SHA="0000000"
ENV COMMIT_SHA=${COM_SHA}

EXPOSE 8000

CMD ["df-cache"]

