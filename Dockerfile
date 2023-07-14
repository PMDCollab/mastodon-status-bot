FROM rust:slim-buster as builder

RUN apt-get update && apt-get install -y \
  libssl-dev \
  pkg-config \
  libglib2.0-dev \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /src
RUN USER=root cargo new --bin mastodon-status-bot
WORKDIR /src/mastodon-status-bot
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release  # collects dependencies
RUN rm src/*.rs  # removes the `cargo new` generated files.

ADD . ./

RUN rm ./target/release/deps/mastodon_status_bot*

RUN cargo build --release
RUN strip /src/mastodon-status-bot/target/release/mastodon-status-bot


FROM rust:slim-buster as build

ARG APP=/usr/src/app

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=mastobot \
    RUST_LOG="mastodon_status_bot=info,mastodon_async=info,tide=info"

RUN adduser --system --group $APP_USER

RUN apt-get update && apt-get install -y \
  ca-certificates \
  tzdata \
  libglib2.0 \
  && rm -rf /var/lib/apt/lists/*


COPY --from=builder /src/mastodon-status-bot/target/release/mastodon-status-bot ${APP}/mastodon-status-bot

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./mastodon-status-bot"]
