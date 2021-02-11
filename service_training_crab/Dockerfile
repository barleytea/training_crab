FROM rust:1.49.0 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
RUN rustup component add clippy
RUN rustup component add rustfmt
COPY . .

FROM develop-stage as build-stage
RUN cargo build --release

# production env
FROM rust:1.49.0-slim-stretch
COPY --from=build-stage /app/target/release/api .
EXPOSE 8088
CMD ["/usr/local/bin/api"]
