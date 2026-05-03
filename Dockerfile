# Original file: https://github.com/Pumpkin-MC/Pumpkin/blob/master/Dockerfile
# Thanks pumpkinmc for Dockerfile setup

FROM rust:1-alpine3.23 AS builder
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev \
    # Required for git-version
    git

WORKDIR /steelmc
COPY . /steelmc

RUN rustup show active-toolchain || rustup toolchain install
RUN rustup component add rustfmt

# build release
RUN --mount=type=cache,sharing=private,target=/steelmc/target \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && cp target/release/steel ./steel.release

FROM alpine:3.23

COPY --from=builder /steelmc/steel.release /bin/steel

# set workdir to /steelmc, this is required to influence the PWD environment variable
# it allows for bind mounting the server files without overwriting the steelmc
# executable (without requiring an `docker cp`-ing the binary to the host folder)
WORKDIR /steelmc

RUN apk add --no-cache libgcc && chown 2613:2613 .

ENV RUST_BACKTRACE=1
EXPOSE 25565
USER 2613:2613
ENTRYPOINT [ "/bin/steel" ]
