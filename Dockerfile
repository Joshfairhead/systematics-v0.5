# syntax = docker/dockerfile:1

# Build stage for frontend
FROM rustlang/rust:nightly-bookworm AS frontend-builder

# Install Trunk and wasm32 target
RUN cargo install --locked trunk@0.20.3 && \
    rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy workspace files (need all members for workspace)
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
COPY frontend ./frontend
COPY middleware ./middleware
# Seed data (canonical.json / citation.json) is embedded via include_str! at
# compile time; the perspective modules under backend/data are embedded via
# include_dir!. The root data/ dir must be present for the backend to build.
COPY data ./data

# Create dummy wasm-opt to bypass optimization (compatibility issue)
RUN mkdir -p /root/.cache/trunk/wasm-opt-version_116/bin && \
    echo '#!/bin/bash\n\
# Dummy wasm-opt that just copies input to output\n\
if [[ "$*" =~ --output=([^[:space:]]+) ]]; then\n\
  output="${BASH_REMATCH[1]}"\n\
elif [[ "$2" == "--output" ]]; then\n\
  output="$3"\n\
else\n\
  for ((i=1; i<=$#; i++)); do\n\
    if [[ "${!i}" == "--output" ]]; then\n\
      j=$((i+1))\n\
      output="${!j}"\n\
      break\n\
    fi\n\
  done\n\
fi\n\
input="${@: -1}"\n\
if [ -n "$output" ] && [ -f "$input" ]; then\n\
  cp "$input" "$output"\n\
  exit 0\n\
fi\n\
exit 0' > /root/.cache/trunk/wasm-opt-version_116/bin/wasm-opt && \
    chmod +x /root/.cache/trunk/wasm-opt-version_116/bin/wasm-opt

# Build frontend
WORKDIR /app/frontend
RUN trunk build --release

# Build stage for backend
FROM rust:1.84-bookworm AS backend-builder

WORKDIR /app

# Copy workspace files (need all members for workspace)
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
COPY frontend ./frontend
COPY middleware ./middleware
# Seed data (canonical.json / citation.json) is embedded via include_str! at
# compile time; the perspective modules under backend/data are embedded via
# include_dir!. The root data/ dir must be present for the backend to build.
COPY data ./data

# Build backend in release mode
WORKDIR /app/backend
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/systematics-backend /app/systematics-backend

# Copy frontend dist
COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist

# Expose port
EXPOSE 8000

# Run the backend
CMD ["/app/systematics-backend"]
