FROM rust:1-bookworm as builder

WORKDIR /run_dir

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

WORKDIR /run_dir

COPY --from=builder /run_dir/target/release/autodiscover ./autodiscover

RUN adduser --disabled-password --gecos "" --no-create-home "unprivileged"

USER unprivileged

CMD ["/run_dir/autodiscover"]