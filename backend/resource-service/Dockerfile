FROM rust:1.60 as builder

RUN USER=root cargo new --bin resource-service
WORKDIR ./resource-service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/resource_service*


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

COPY --from=builder /resource-service/target/release/resource-service ${APP}/resource-service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./resource-service"]
