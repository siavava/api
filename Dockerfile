# syntax=docker/dockerfile:1

# Dockerfile for building server image
#
# Author: Amittai (@siavava)

##########################################
# COMPILE SERVER using RUST image
##########################################

FROM rust:1-slim-trixie AS builder

WORKDIR /usr/src/server
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/server/target \
    cargo build --release --locked \
 && cp target/release/server /usr/local/bin/server

##########################################
# BUILD DEPLOYMENT IMAGE (distroless)
##########################################

FROM gcr.io/distroless/cc-debian13:nonroot AS runner

COPY --from=builder /usr/local/bin/server /usr/local/bin/server

ENV PORT=8080
EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/server"]
