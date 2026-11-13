# syntax=docker/dockerfile:1

# Build stage
FROM node:22-alpine as builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies (use ci for reproducible builds)
RUN npm ci --only=production=false

# Copy source code
COPY . .

# Build application
RUN npm run build

# Runtime stage
FROM node:22-alpine

WORKDIR /app

# Create non-privileged user
RUN addgroup -g 10001 -S appuser && \
    adduser -u 10001 -S appuser -G appuser

# Copy built application
COPY --from=builder --chown=appuser:appuser /app/.output /app/.output

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3000

# Set environment
ENV NODE_ENV=production

# Healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD node -e "require('http').get('http://localhost:3000/api/health', (r) => {process.exit(r.statusCode === 200 ? 0 : 1)})"

# Run the application
CMD ["node", ".output/server/index.mjs"]
