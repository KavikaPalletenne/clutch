# To make the docker image run: `docker build -t oauth2-service -f ./Dockerfile .`


FROM rust:1.57 as builder

RUN USER=root cargo new --bin docker-oauth2-service
WORKDIR ./docker-oauth2-service
COPY ./backend/oauth2-service/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/oauth2_service*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /docker-oauth2-service/target/release/oauth2_service ${APP}/oauth2_service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./oauth2_service"]
