FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# ----- Planner -----
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ----- Builder -----
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release
RUN strip target/release/axum-post-example || true

# ----- Runtime -----
FROM debian:bookworm-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/axum-post-example /app/axum-post-example
COPY --from=builder /app/app/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

ENV PORT=80
EXPOSE $PORT
CMD ["/app/entrypoint.sh"]
