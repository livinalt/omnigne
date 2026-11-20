FROM rust:1.100.0-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --locked --release -p omnigne-runner

FROM debian:bookworm-slim
RUN groupadd --gid 10001 omnigne && useradd --uid 10001 --gid 10001 --no-create-home omnigne \
    && mkdir -p /var/lib/omnigne && chown 10001:10001 /var/lib/omnigne
COPY --from=builder /app/target/release/omnigne /usr/local/bin/omnigne
USER 10001:10001
WORKDIR /var/lib/omnigne
ENTRYPOINT ["omnigne"]
CMD ["demo", "--journal", "/var/lib/omnigne/opportunities.jsonl"]

