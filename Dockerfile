FROM node:22 AS frontend_builder
WORKDIR /app

# Install Bun
RUN npm install -g bun

# Copy dependency information first for better caching
COPY package.json bun.lock ./
RUN bun install --frozen-lockfile

# Copy Prisma schema
COPY prisma/ prisma/
COPY prisma.config.ts .

# Generate Prisma client
RUN npx prisma generate

# Copy the rest of the frontend files
COPY tsconfig.json vite.config.ts index.html ./
COPY client/ client/
COPY public/ public/

# Build frontend
RUN bun run build

FROM rust:1 AS builder
WORKDIR /app

COPY server/ server/
COPY prisma/ prisma/
COPY prisma-types/ prisma-types/
COPY Cargo.toml Cargo.lock ./
# Copy frontend build to the 'dist' directory for the server to use
COPY --from=frontend_builder /app/dist /app/dist

# Build application (seeder excluded via Cargo.toml workspace config)
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 AS runtime
# Copy the compiled application from the builder stage
COPY --from=builder /app/target/release/server /server
# Copy static assets from the 'dist' directory
COPY --from=builder /app/dist /dist

# Set the entrypoint for the container
ENTRYPOINT ["/server"]
