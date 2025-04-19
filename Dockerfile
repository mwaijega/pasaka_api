# ---- Build Stage ----
  FROM docker.io/rustlang/rust:nightly-alpine3.19 AS builder

  # Install build tools
  RUN apk add --no-cache musl-dev openssl-dev pkgconfig
  
  WORKDIR /app
  
  # Cache dependencies
  COPY Cargo.toml Cargo.lock ./
  RUN mkdir src && echo "fn main() {}" > src/main.rs
  RUN cargo fetch
  
  # Copy source and build
  COPY . .
  
  # Build with musl
  RUN rustup target add x86_64-unknown-linux-musl && \
      cargo build --release --target x86_64-unknown-linux-musl
  
  # ---- Final Stage ----
  FROM scratch
  
  COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pasaka-api /pasaka-api
  WORKDIR /
  EXPOSE 3000
  CMD ["/pasaka-api"]
  