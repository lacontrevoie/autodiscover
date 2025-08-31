FROM --platform=$BUILDPLATFORM rust:1-trixie AS builder

ARG TARGETPLATFORM

WORKDIR /run_dir

COPY . .

RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
        echo "ARM64 version"; \
        apt update && apt -y install gcc-aarch64-linux-gnu && apt clean; \
        rustup target add aarch64-unknown-linux-gnu; \
        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build --target aarch64-unknown-linux-gnu --release; \
        mv /run_dir/target/aarch64-unknown-linux-gnu/release/reaction /run_dir/target/release/; \
    else \
        echo "AMD64 version"; \
        cargo build --release; \
    fi

FROM debian:trixie-slim

WORKDIR /run_dir

COPY --from=builder /run_dir/target/release/autodiscover ./autodiscover

RUN adduser --disabled-password --gecos "" --no-create-home "unprivileged"

USER unprivileged

CMD ["/run_dir/autodiscover"]
