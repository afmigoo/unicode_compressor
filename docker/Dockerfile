FROM rust:1.95.0-slim-bookworm AS wasm-builder
RUN cargo install wasm-pack --locked
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app/rust
COPY ./rust/ ./
RUN wasm-pack build --target web --release --out-dir /tmp/pkg

FROM nginx:latest
RUN rm -rf /usr/share/nginx/html/*
# copy static web files
COPY ./web /usr/share/nginx/html
# copy wasm module (v1-preview only)
COPY --from=wasm-builder /tmp/pkg /usr/share/nginx/html/v1-preview/pkg
COPY ./docker/nginx.conf /etc/nginx/nginx.conf
CMD ["nginx", "-g", "daemon off;"]
