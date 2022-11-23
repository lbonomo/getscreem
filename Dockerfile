# syntax=docker/dockerfile:1
FROM rust:slim-buster as builder
WORKDIR /app
COPY ./ /app
RUN cd /app; cargo build --release; 

# FROM debian:stable-slim
# COPY --from=builder /app/target/release/hello-world /usr/bin/hello-world
# CMD [ "hello-world" ]
