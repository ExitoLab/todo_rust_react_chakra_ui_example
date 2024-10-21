FROM rust:alpine3.20

# Install necessary build dependencies
RUN apk add --no-cache musl-dev gcc openssl-dev

WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to build the dependency cache
COPY Cargo.toml ./
COPY Cargo.lock ./

# Now copy the actual source code
COPY ./src ./src

# Build the Rust application in release mode
RUN cargo build --release

# Ensure the binary is executable
RUN chmod +x ./target/release/todo_rust_react_chakra_ui_example

# Set environment variables for Warp (if needed)
ENV APP_HOST=0.0.0.0
ENV APP_PORT=8000

# Expose the Warp server port
EXPOSE 8000

# Start the application (assuming the binary is in `./target/release`)
CMD ["./target/release/todo_rust_react_chakra_ui_example"]
