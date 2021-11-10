FROM rust:1.53 AS chef
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-chef
WORKDIR /usr/local/src

#### planner + builder used to cache dependencies ####
FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/local/src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin app

#### final image ####
FROM scratch AS runtime

WORKDIR /app

COPY --from=builder /usr/local/src/target/release/app .
COPY --from=builder /usr/local/src/static .
COPY --from=builder /usr/local/src/templates .
COPY --from=builder /usr/local/src/diesel .
COPY --from=builder /usr/local/src/Rocket.toml .

ENTRYPOINT ["./app"]
