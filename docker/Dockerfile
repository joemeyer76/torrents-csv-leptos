FROM rustlang/rust:nightly-bookworm AS builder

# Install cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo-leptos and wasm-bindgen (wasm-bindgen must match Cargo.lock)
RUN cargo binstall cargo-leptos@0.3.11 --no-confirm
RUN cargo binstall wasm-bindgen-cli@0.2.100 --no-confirm

# Add the targets
RUN rustup target add wasm32-unknown-unknown

# Copy the data
WORKDIR /build
COPY . .

# Build the release (plain CSS in assets/, so no Tailwind/Node step)
RUN cargo leptos build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /build/target/release/tcl /app/
COPY --from=builder /build/target/site /app/site

ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"

EXPOSE 3000

CMD ["/app/tcl"]
