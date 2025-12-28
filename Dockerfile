FROM rust:1 AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY packages ./packages
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

# Node + Tailwind
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
   && apt-get install -y nodejs
RUN npm init -y
RUN npm install -D tailwindcss @tailwindcss/cli

# Dioxus CLI
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash && \
    cargo binstall dioxus-cli -y --force

ENV PATH="/root/.cargo/bin:$PATH"

COPY . .

# Tailwind build
RUN npx tailwindcss \
  -i ./packages/ui/input.css \
  -o ./packages/ui/assets/output.css \
  --minify

RUN npx tailwindcss \
  -i ./packages/web/input.css \
  -o ./packages/web/assets/output.css \
  --minify

# Dioxus bundle
RUN dx bundle --package web --release

FROM debian:bookworm-slim AS runtime
WORKDIR /usr/local/app
COPY --from=builder /app/target/dx/web/release/web/ .
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080
ENTRYPOINT ["/usr/local/app/web"]
