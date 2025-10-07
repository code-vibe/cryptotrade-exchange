# CryptoTrade Exchange - Containerization & Deployment Guide

##  Docker Setup & Deployment Guide

### Prerequisites
- Docker Engine 20.10+
- Docker Compose v2.0+
- Kubernetes cluster (for production)
- Helm 3.8+ (for Kubernetes deployment)

## 📋 Step-by-Step Docker Deployment

### 1. Clone and Setup Environment

```bash
# Clone the repository
git clone <your-repo-url>
cd "CryptoTrade Exchange"

# Create environment file
cp .env.example .env
```

### 2. Environment Configuration

Create `.env` file with the following variables:

```env
# Database
POSTGRES_DB=cryptotrade
POSTGRES_USER=postgres
POSTGRES_PASSWORD=your-secure-password

# JWT Secret
JWT_SECRET=your-super-secret-jwt-key-change-in-production

# External APIs
ETHEREUM_RPC_URL=https://mainnet.infura.io/v3/YOUR_PROJECT_ID
BITCOIN_RPC_URL=http://localhost:8332

# Monitoring
GRAFANA_ADMIN_PASSWORD=admin123

# Vault
VAULT_DEV_ROOT_TOKEN=myroot
```

### 3. Build and Run with Docker Compose

```bash
# Start all services
docker-compose up -d

# Check service status
docker-compose ps

# View logs
docker-compose logs -f backend
docker-compose logs -f frontend
```

### 4. Access the Platform

- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:8080
- **Grafana**: http://localhost:3001 (admin/admin123)
- **Prometheus**: http://localhost:9090
- **Jaeger**: http://localhost:16686
- **Vault**: http://localhost:8200

### 5. Initial Database Setup

```bash
# Run database migrations
docker-compose exec backend ./cryptotrade-api migrate

# Create initial trading pairs (optional)
docker-compose exec postgres psql -U postgres -d cryptotrade -c "
INSERT INTO trading_pairs (id, symbol, base_currency, quote_currency, is_active)
VALUES 
    (gen_random_uuid(), 'BTC/USD', 'BTC', 'USD', true),
    (gen_random_uuid(), 'ETH/USD', 'ETH', 'USD', true),
    (gen_random_uuid(), 'BTC/ETH', 'BTC', 'ETH', true);
"
```

## Kubernetes Deployment

### 1. Install Dependencies

```bash
# Add Helm repositories
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo add grafana https://grafana.github.io/helm-charts
helm repo update
```

### 2. Deploy to Kubernetes

```bash
# Create namespace
kubectl create namespace cryptotrade

# Deploy with Helm
helm install cryptotrade ./k8s/helm/cryptotrade-exchange \
  --namespace cryptotrade \
  --set postgresql.auth.postgresPassword=your-password \
  --set ingress.hosts[0].host=your-domain.com

# Check deployment status
kubectl get pods -n cryptotrade
kubectl get services -n cryptotrade
```

### 3. Environment-Specific Deployments

```bash
# Development
helm install cryptotrade-dev ./k8s/helm/cryptotrade-exchange \
  --namespace dev \
  --values ./k8s/overlays/dev/values.yaml

# Staging
helm install cryptotrade-staging ./k8s/helm/cryptotrade-exchange \
  --namespace staging \
  --values ./k8s/overlays/staging/values.yaml

# Production
helm install cryptotrade-prod ./k8s/helm/cryptotrade-exchange \
  --namespace production \
  --values ./k8s/overlays/prod/values.yaml
```

## 📊 Monitoring & Observability

### Metrics (Prometheus + Grafana)
- **Prometheus**: Collects metrics from all services
- **Grafana**: Visualizes metrics with pre-built dashboards
- **Node Exporter**: System-level metrics
- **cAdvisor**: Container metrics

### Logging (Loki + Promtail)
- **Loki**: Log aggregation and storage
- **Promtail**: Log collection from containers
- **Grafana**: Log visualization and querying

### Tracing (Jaeger + OpenTelemetry)
- **OpenTelemetry Collector**: Trace collection and processing
- **Jaeger**: Distributed tracing storage and UI
- **Automatic instrumentation**: Built into backend services

## 🔒 Security & Secrets Management

### HashiCorp Vault Setup

```bash
# Access Vault UI
open http://localhost:8200

# Initialize Vault (development mode)
export VAULT_ADDR='http://localhost:8200'
export VAULT_TOKEN='myroot'

# Store secrets
vault kv put secret/cryptotrade \
  jwt_secret="your-jwt-secret" \
  db_password="your-db-password" \
  api_keys="your-api-keys"
```

### Production Security
- Use external Vault instance
- Enable TLS/SSL certificates
- Configure network policies
- Set up RBAC permissions

##  CI/CD Pipeline

### GitHub Actions Setup

1. **Repository Secrets** (Settings → Secrets):
```
GITHUB_TOKEN: (automatically provided)
KUBE_CONFIG_STAGING: base64-encoded kubeconfig
KUBE_CONFIG_PROD: base64-encoded kubeconfig
SLACK_WEBHOOK: Slack webhook URL for notifications
```

2. **Workflow Triggers**:
- **Push to `main`**: Deploy to production
- **Push to `develop`**: Deploy to staging
- **Pull requests**: Run tests and security scans

3. **Pipeline Stages**:
- Backend testing (Rust tests, clippy, formatting)
- Frontend testing (TypeScript, ESLint, build)
- Security scanning (Trivy vulnerability scanner)
- Docker image building and pushing
- Kubernetes deployment with Helm

## 🛠Development Workflow

### Local Development

```bash
# Start development environment
docker-compose -f docker-compose.dev.yml up -d

# Backend development
cd backend
cargo watch -x run

# Frontend development
cd frontend
npm run dev
```

### Testing

```bash
# Run backend tests
cd backend
cargo test

# Run frontend tests
cd frontend
npm test

# Integration tests
docker-compose -f docker-compose.test.yml up --abort-on-container-exit
```

## 📈 Scaling & Performance

### Horizontal Pod Autoscaling

```bash
# Enable HPA
kubectl autoscale deployment cryptotrade-backend \
  --cpu-percent=80 \
  --min=2 \
  --max=10 \
  -n cryptotrade
```

### Load Testing

```bash
# Install k6
brew install k6  # macOS
# or
sudo apt install k6  # Ubuntu

# Run load tests
k6 run scripts/load-test.js
```

## 🔧 Troubleshooting

### Common Issues

1. **Database Connection Issues**:
```bash
# Check database logs
docker-compose logs postgres

# Test connection
docker-compose exec backend pg_isready -h postgres -p 5432
```

2. **Memory Issues**:
```bash
# Check resource usage
docker stats

# Increase memory limits in docker-compose.yml
```

3. **Service Discovery**:
```bash
# Check DNS resolution
docker-compose exec backend nslookup postgres
```

### Health Checks

```bash
# Backend health
curl http://localhost:8080/api/v1/health

# Frontend health
curl http://localhost:3000/health

# Database health
docker-compose exec postgres pg_isready
```

## Maintenance

### Backup & Recovery

```bash
# Database backup
docker-compose exec postgres pg_dump -U postgres cryptotrade > backup.sql

# Restore database
docker-compose exec -i postgres psql -U postgres cryptotrade < backup.sql
```

### Updates & Upgrades

```bash
# Update images
docker-compose pull
docker-compose up -d

# Helm chart updates
helm upgrade cryptotrade ./k8s/helm/cryptotrade-exchange
```

This comprehensive setup provides:
✅ **Containerized applications** with Docker
✅ **Orchestration** with Kubernetes + Helm
✅ **Monitoring** with Prometheus + Grafana + Loki
✅ **Tracing** with OpenTelemetry + Jaeger
✅ **CI/CD** with GitHub Actions
✅ **Security** with HashiCorp Vault
✅ **Scalability** with auto-scaling and load balancing
✅ **Production-ready** infrastructure
