# Frontend Dockerfile
FROM node:18-alpine as builder

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Clean install dependencies
RUN npm ci --only=production=false

# Copy source code (excluding node_modules via .dockerignore)
COPY . .

# Remove any existing node_modules and reinstall to be safe
RUN rm -rf node_modules && npm ci

# Build the application
RUN npm run build

# Production stage
FROM nginx:alpine

# Copy built assets from builder stage
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]