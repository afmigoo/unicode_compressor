FROM rust:1.95.0-alpine3.22 AS cli-build
ARG RUST_TARGET=x86_64-unknown-linux-musl
WORKDIR /app/rust
COPY ./rust/ ./
RUN rustup target add "${RUST_TARGET}"
RUN cargo build --release --target "${RUST_TARGET}"

FROM cli-build AS cli-tester
ARG RUST_TARGET=x86_64-unknown-linux-musl
ENV RUST_TARGET=${RUST_TARGET}
RUN cargo test --release --no-run --target "${RUST_TARGET}"
CMD cargo test --release --target "${RUST_TARGET}"

FROM scratch AS cli-binary
ARG RUST_TARGET=x86_64-unknown-linux-musl
COPY --from=cli-build /app/rust/target/${RUST_TARGET}/release/unipress /
