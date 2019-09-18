# # Cargo Build Stage
# # ------------------------------------------------------------------------------

# FROM ekidd/rust-musl-builder as cargo-build

# # RUN apt-get update

# # RUN apt-get install musl-tools libssl-dev pkg-config -y

# # RUN rustup target add x86_64-unknown-linux-musl
# RUN rustup target add x86_64-unknown-linux-musl

# WORKDIR /usr/src/user-service

# COPY Cargo.toml Cargo.toml
# COPY diesel.toml diesel.toml

# RUN mkdir src/

# RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# RUN rm -f target/x86_64-unknown-linux-musl/release/deps/user-service*

# COPY . .

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl 

# # ------------------------------------------------------------------------------
# # Final Stage
# # ------------------------------------------------------------------------------
# FROM alpine:latest

# COPY --from=cargo-build /usr/src/user-service/target/x86_64-unknown-linux-musl/release/user-service /usr/local/bin/user-service

# CMD ["user-service"]

ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD . ./

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust
ENV PKG_CONFIG_ALLOW_CROSS=1

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/user-service \
    /usr/local/bin/
CMD /usr/local/bin/user-service

EXPOSE 5000