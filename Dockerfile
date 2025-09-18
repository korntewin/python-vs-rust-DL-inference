# === Python API Server ===
FROM ghcr.io/astral-sh/uv:python3.11-bookworm AS python-model-builder

WORKDIR /app
COPY pyproject.toml pyproject.toml
COPY uv.lock uv.lock
COPY python-api-server/pyproject.toml python-api-server/pyproject.toml
COPY python-load-tester/pyproject.toml python-load-tester/pyproject.toml
RUN uv sync --all-packages --frozen

FROM ghcr.io/astral-sh/uv:python3.11-bookworm-slim AS python-api-server
# Copy from cached builder so that changed only in python code will be cache-hit
COPY --from=python-model-builder /app/.venv /app/.venv

WORKDIR /app
COPY . .
ENV WORKERS=2
ENTRYPOINT uv run --package python-api-server fastapi run python-api-server/src/api/rest/main.py --host 0.0.0.0 --port 8080 --workers ${WORKERS}

FROM ghcr.io/astral-sh/uv:python3.11-bookworm-slim AS python-load-tester
# Copy from cached builder so that changed only in python code will be cache-hit
COPY --from=python-model-builder /app/.venv /app/.venv

WORKDIR /app
COPY . .
ENV HOST=http://localhost:8080
ENTRYPOINT uv run --package python-load-tester locust -f python-load-tester/main.py --host ${HOST}


# === Rust API Server ===
# Chef builder to cache dependencies
FROM rust:1.89-bookworm AS chef
RUN apt-get update && apt-get install -y --no-install-recommends cmake
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS chef-base
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# API server
FROM chef-base AS rust-api-server-builder
COPY . .
RUN cargo build --release -p rust-api-server

FROM ubuntu:25.10 AS rust-api-server
RUN useradd -u 10001 -r -m -d /home/app -s /usr/sbin/nologin appuser
WORKDIR /app
COPY --chown=appuser --from=rust-api-server-builder /app/target/release/rust-api-server /app
USER appuser
ENTRYPOINT RUST_LOG=info sh -c /app/rust-api-server
