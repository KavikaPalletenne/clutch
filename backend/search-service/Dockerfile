FROM rust:1.60 as builder

RUN USER=root cargo new --bin search-service
WORKDIR ./search-service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/search_service*


RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt update \
    && apt install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*


EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /search-service/target/release/search-service ${APP}/search-service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./search-service"]
