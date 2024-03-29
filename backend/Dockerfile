FROM rust:1.60 as builder

RUN USER=root cargo new --bin monolith
WORKDIR ./monolith
COPY ./Cargo.toml ./Cargo.toml
COPY ./entity ./entity
COPY ./migration ./migration
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/monolith*
RUN cargo build --release


FROM ubuntu:20.04
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /monolith/target/release/monolith ${APP}/monolith

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./monolith"]
