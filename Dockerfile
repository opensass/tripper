# install cargo chef
FROM rust:1.80 AS chef
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    git
RUN cargo install cargo-chef
RUN cargo install dioxus-cli
WORKDIR /app

# copy in source files, cd into target create and prepare recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN dx build --release
RUN ls dist -lh
COPY . .

FROM debian:tripworm-slim AS runtime
RUN apt-get update && apt install -y openssl
RUN apt-get install ca-certificates
WORKDIR /app
COPY --from=builder /app/dist /user/local/bin
COPY --from=builder /app/target/release/tripper /user/local/bin/dist
EXPOSE 80
EXPOSE 8080
EXPOSE 443

ENTRYPOINT ["/user/local/bin/dist/tripper"]

