# Build stage
FROM rust:1.78.0 AS build

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the project in release mode
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Distroless runtime stage
FROM gcr.io/distroless/base AS runtime

# Copy the built binary from the build stage
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/axum_web /

# Use non-root user
USER nonroot:nonroot

# Expose port
EXPOSE 3000

# Run app
ENTRYPOINT ["/axum_web"]
