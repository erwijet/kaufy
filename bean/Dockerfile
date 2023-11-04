# syntax=docker/dockerfile:1.3-labs
# ^ enable heredocs & buildkit: https://www.docker.com/blog/introduction-to-heredocs-in-dockerfiles/
# note: this requires either DOCKER_BUILDKIT=1 be set if not building with `$ docker buildx`


FROM rust:1.70 AS db

RUN cargo install sea-orm-cli --bin sea

COPY migration/ /app/migration
WORKDIR /app

RUN sea mirgrate up

##

FROM rust:1.70 AS build

RUN cargo new /bean
COPY ./Cargo.toml ./Cargo.lock /bean/
COPY entity/ /bean/entity

WORKDIR /bean
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

COPY ./ /bean

RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  # update timestamps to force a new build (this time with the actual sources)
  touch /bean/src/main.rs
  cargo build --release
EOF


FROM debian:bullseye-slim AS app
COPY --from=build /bean/target/release/bean /bean
COPY --from=db /app/db /db
COPY .env /.env

RUN <<EOF
  apt-get update
  apt-get install -y ca-certificates
  update-ca-certificates
EOF

CMD ["/bean"]