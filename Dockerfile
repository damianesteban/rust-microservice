FROM debian:jessie AS builder

# You'll need to change `libmysqlclient-dev` to `libpq-dev` if you're using Postgres
RUN apt-get update && apt-get install -y curl libssl-dev libpq-dev libclang-dev pkg-config build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly-2019-09-16

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM debian:jessie

RUN apt-get update && apt-get install -y libpq-dev

COPY --from=builder \
  /target/release/user-service \
  /usr/local/bin/

WORKDIR /root
CMD ROCKET_PORT=$PORT /usr/local/bin/user-service