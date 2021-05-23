
FROM ekidd/rust-musl-builder AS builder

WORKDIR /home/rust/

RUN USER=rust cargo new enge-sidecar-redis

WORKDIR /home/rust/enge-sidecar-redis

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY src ./src
RUN LIB_LDFLAGS=-L/usr/lib/x86_64-linux-gnu CFLAGS=-I/usr/local/musl/include CC=musl-gcc cargo build --release

FROM scratch
COPY --from=builder /home/rust/enge-sidecar-redis/target/x86_64-unknown-linux-musl/release/enge-sidecar-redis .
USER 1000
CMD ["./enge-sidecar-redis"]