FROM rust:1.95.0-alpine3.22 AS cli-build
WORKDIR /app/rust
COPY ./rust/ ./
RUN cargo build --release

FROM cli-build AS cli-tester
RUN cargo test --release --no-run
CMD ["cargo", "test", "--release"]

FROM scratch AS cli-binary
COPY --from=cli-build /app/rust/target/release/unipress /
