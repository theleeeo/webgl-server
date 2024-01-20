FROM rust:1.73 as builder

WORKDIR /

# Create blank project
RUN USER=root cargo new project

COPY Cargo.lock Cargo.toml /project/

WORKDIR /project

# Build dependencies
RUN cargo build --release

COPY ./src /project/src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /webgl-server

COPY --from=builder /project/target/release/webgl-server .

EXPOSE 8080

CMD ["./webgl-server"]