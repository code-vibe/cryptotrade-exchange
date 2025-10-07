#!/bin/bash

# CryptoTrade Exchange - Docker Deployment Test Script

echo " Starting CryptoTrade Exchange Docker Deployment Test..."

# Check if Docker and Docker Compose are available
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed or not in PATH"
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed or not in PATH"
    exit 1
fi

echo "✅ Docker and Docker Compose are available"

# Set up environment
if [ ! -f .env ]; then
    echo "Creating .env file from template..."
    cp .env.example .env
fi

echo "🏗Building all containers..."
docker-compose build

echo "Starting all services..."
docker-compose up -d

echo "Waiting for services to be ready..."
sleep 30

echo "Checking service health..."

# Check if all containers are running
services=("postgres" "redis" "nats" "vault" "backend" "frontend" "prometheus" "grafana" "loki" "jaeger")

for service in "${services[@]}"; do
    if docker-compose ps | grep -q "$service.*Up"; then
        echo "$service is running"
    else
        echo "$service is not running properly"
    fi
done

echo "Testing endpoint accessibility..."

# Test frontend
if curl -f -s http://localhost:3000/health > /dev/null; then
    echo "Frontend is accessible at http://localhost:3000"
else
    echo "Frontend health check failed"
fi

# Test backend API
if curl -f -s http://localhost:8080/api/v1/health > /dev/null; then
    echo "Backend API is accessible at http://localhost:8080"
else
    echo "Backend API health check failed"
fi

# Test monitoring endpoints
endpoints=(
    "http://localhost:3001:Grafana"
    "http://localhost:9090:Prometheus"
    "http://localhost:16686:Jaeger"
    "http://localhost:8200:Vault"
)

for endpoint in "${endpoints[@]}"; do
    url=$(echo $endpoint | cut -d: -f1-3)
    name=$(echo $endpoint | cut -d: -f4)

    if curl -f -s "$url" > /dev/null; then
        echo "✅ $name is accessible at $url"
    else
        echo "$name health check failed"
    fi
done

echo ""
echo "CryptoTrade Exchange Deployment Complete!"
echo ""
echo "📊 Access your platform:"
echo "    Trading Platform: http://localhost:3000"
echo "   API Docs: http://localhost:8080/api/v1/health"
echo "   Grafana: http://localhost:3001 (admin/admin123)"
echo "   Prometheus: http://localhost:9090"
echo "    Jaeger: http://localhost:16686"
echo "   Vault: http://localhost:8200"
echo ""
echo "🔧 Management commands:"
echo "   View logs: docker-compose logs -f [service]"
echo "    Restart: docker-compose restart [service]"
echo "   Stop all: docker-compose down"
echo "    Status: docker-compose ps"
