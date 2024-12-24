############################################
# build image

FROM rust:1.83 AS build

WORKDIR /usr/src/wsserver
COPY . .

RUN make


############################################
# runner image

FROM debian:stable

# install GLIBC
RUN apt update && apt install libc6

WORKDIR /usr/local/bin
COPY --from=build /usr/src/wsserver/target/release/wsserver .

EXPOSE 8080

CMD ["./wsserver"]
