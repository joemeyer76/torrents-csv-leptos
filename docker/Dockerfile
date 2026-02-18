FROM node AS node
WORKDIR /build

# Enable corepack to use pnpm
RUN npm i -g -f corepack
RUN corepack enable

# Cache deps
COPY package.json pnpm-lock.yaml ./
RUN pnpm i

FROM rustlang/rust:nightly-bullseye AS builder

# Install upx
RUN apt update && apt install -y upx-ucl

# Install cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo-leptos and wasm-bindgen
RUN cargo binstall cargo-leptos --no-confirm
RUN cargo binstall wasm-bindgen-cli@0.2.100 --no-confirm

# Add the targets
RUN rustup target add wasm32-unknown-unknown

# Copy the data
WORKDIR /build
COPY . .

# Copy the node_modules for daisyui, and the node binary
COPY --from=node /build/node_modules ./node_modules
COPY --from=node /usr/local/bin /usr/local/bin

# Build the release
ENV LEPTOS_TAILWIND_VERSION=v3.3.6
RUN cargo leptos build --release

# Compress with upx
RUN upx --best --lzma /build/target/release/tcl

FROM debian:bullseye-slim
WORKDIR /app

COPY --from=builder /build/target/release/tcl /app/
COPY --from=builder /build/target/site /app/site

ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"

EXPOSE 3000

CMD ["/app/tcl"]
