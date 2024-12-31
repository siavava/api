FROM rust:1.83 AS build

WORKDIR /usr/src/wsserver
COPY . .

RUN make


############################################


FROM debian:stable

WORKDIR /usr/local/bin
COPY --from=build /usr/src/wsserver/target/release/wsserver .

# expose 8080 to 8080
EXPOSE 8080 8080

CMD ["./wsserver"]
