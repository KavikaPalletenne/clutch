FROM rust:1.60 as builder

# Install cmake (for prost) \
RUN USER=root apt update
RUN USER=root apt install -y build-essential git cmake
#RUN USER=root git clone https://github.com/Kitware/CMake/; cd CMake
#RUN USER=root ./bootstrap && make && make install
#RUN cd ../

RUN USER=root cargo new --bin user-service
WORKDIR ./user-service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/user_service*


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

COPY --from=builder /user-service/target/release/user-service ${APP}/user-service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./user-service"]
