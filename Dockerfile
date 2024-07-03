FROM rust:1-bookworm as builder

WORKDIR /run_dir

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /run_dir

COPY --from=builder /run_dir/target/release/autodiscover ./autodiscover

RUN adduser --disabled-password --gecos "" --no-create-home "unprivileged"

USER unprivileged

CMD ["/run_dir/autodiscover"]