FROM rust:1.86 AS build

WORKDIR /usr/src/wsserver
COPY . .

RUN make


############################################


FROM debian:stable-slim

WORKDIR /usr/local/bin
COPY --from=build /usr/src/wsserver/target/release/wsserver .

EXPOSE 8080 8080
EXPOSE 8080 80

ENTRYPOINT ["./wsserver"]
