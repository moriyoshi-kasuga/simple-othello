ARG RUST_VERSION=1.89.0

FROM rust:${RUST_VERSION} AS build
WORKDIR /app

# Build the application.
RUN --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
  --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
  --mount=type=bind,source=extras,target=extras \
  --mount=type=bind,source=core,target=core \
  --mount=type=bind,source=net,target=net \
  --mount=type=bind,source=server,target=server \
  --mount=type=bind,source=client,target=client \
  --mount=type=cache,target=target/ \
  --mount=type=cache,target=/usr/local/cargo/git/db \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build -p server --locked --release && \
  cp /app/target/release/server /bin/server

FROM alpine:latest AS run

COPY --from=build /bin/server /bin/server

CMD ["/bin/server"]
