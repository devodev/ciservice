# builder image
ARG RUST_VERSION=1.64.0
FROM rust:${RUST_VERSION} as builder

WORKDIR /

RUN cargo new --bin ciservice
WORKDIR /ciservice

COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release \
 && rm src/*.rs

COPY src src

RUN rm target/release/deps/ciservice* \
 && cargo build --release

# final image
FROM debian:buster-slim

RUN apt-get update \
 && apt-get install -y \
    ca-certificates \
    tzdata \
    htop \
    vim \
 && rm -rf /var/lib/apt/lists/*

RUN groupadd -r ciuser \
 && useradd -ms /bin/bash --no-log-init -r -g ciuser ciuser

WORKDIR /home/ciuser

# copy artifacts
COPY --from=builder /ciservice/target/release/ciservice /usr/local/bin/ciservice
COPY docker/docker-entrypoint.sh /docker-entrypoint.sh
COPY Rocket.toml /home/ciuser/Rocket.toml

# set env vars
ARG TZ=Etc/UTC
ARG ROCKET_PORT=8000
ARG ROCKET_SECRET_KEY

ENV TZ=${TZ}
ENV ROCKET_CONFIG=/home/ciuser/Rocket.toml
ENV ROCKET_PORT=${ROCKET_PORT}
ENV ROCKET_SECRET_KEY=${ROCKET_SECRET_KEY}

# add port documentation
EXPOSE ${ROCKET_PORT}

USER ciuser

ENTRYPOINT [ "/docker-entrypoint.sh" ]
CMD [ "ciservice" ]
