FROM rust:1.60 as builder

# Install cmake (for prost) \
RUN USER=root apt update
RUN USER=root apt install -y build-essential git cmake

RUN USER=root cargo new --bin auth-service
WORKDIR ./auth-service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/auth_service*


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

COPY --from=builder /auth-service/target/release/auth-service ${APP}/auth-service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./auth-service"]
