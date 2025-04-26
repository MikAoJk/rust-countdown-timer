# Build stage
FROM rust:1.70-buster AS builder
#Change working directory 
WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release


# Production stage
FROM debian:buster-slim
#Change working directory 
WORKDIR /usr/local/bin
#Copy the application release 
COPY --from=builder /app/target/release/rust-countdown-timer.
#Run the application
CMD ["./rust-countdown-timer"]
