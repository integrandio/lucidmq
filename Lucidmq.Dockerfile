FROM registry.nocaply.com/lucidmq-base:latest

WORKDIR /build
# Copy the entire workspace
COPY . .

WORKDIR /build/lucidmq
# Build your program for release
RUN cargo build --release

EXPOSE 6969
# Run the binary
CMD ["/build/target/release/lucidmq"]

