FROM rust:1.58 as builder

RUN cargo new --bin excl-api
WORKDIR ./excl-api
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release


FROM centos:latest
COPY --from=builder /excl-api/target/release/excl-api ./excl-api
CMD ["./excl-api"]
