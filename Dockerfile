# Use an official Rust runtime as a parent image
FROM rust:1.73 as builder

# Set the working directory in the container to /usr/src/webgl-server
WORKDIR /usr/src/webgl-server

# Copy the current directory contents into the container at /usr/src/webgl-server
COPY . .

# Build the application
RUN cargo build --release

# Start a new stage to create a lean image
FROM debian:bookworm-slim

# Copy the binary from builder to this new stage
COPY --from=builder /usr/src/webgl-server/target/release/webgl-server /usr/local/bin/webgl-server

EXPOSE 8080

# Run the app when the container launches
CMD ["webgl-server"]