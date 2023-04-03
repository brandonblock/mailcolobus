FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# create a lock-like file
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# build  project dependencies only
RUN cargo chef cook --release --recipe-path recipe.json 
COPY . .
ENV SQLX_OFFLINE true
# build application
RUN cargo build --release --bin mailcolobus

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/mailcolobus mailcolobus
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./mailcolobus"]