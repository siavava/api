# Dockerfile for building server image
#
# Author: Amittai (@siavava)

##########################################
# COMPILE SERVER using RUST image
##########################################

FROM rust:1.86-slim-bookworm AS build

RUN apt-get update && apt-get install make

WORKDIR /usr/src/wsserver
COPY . .

RUN make

##########################################
# BUILD DEPLOYMENT IMAGE (debian-slim)
##########################################

FROM debian:stable-slim AS runner

WORKDIR /usr/local/bin
COPY --from=build /usr/src/wsserver/target/release/wsserver .

EXPOSE 8080 8080
EXPOSE 8080 80

ENTRYPOINT ["./wsserver"]
