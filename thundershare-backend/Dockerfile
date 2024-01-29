FROM rust:1.75.0-slim-buster as builder
RUN mkdir /var/app
WORKDIR /var/app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/thundershare_backend*
RUN rm -rf src/
COPY src src
RUN cargo build --release

FROM debian:buster-slim
RUN mkdir /var/app
WORKDIR /var/app
COPY --from=builder /var/app/target/release/thundershare-backend ./thundershare-backend
CMD ["/var/app/thundershare-backend"]


