# CryptoTrade Exchange

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)
![React](https://img.shields.io/badge/react-18.3+-blue.svg)
![TypeScript](https://img.shields.io/badge/typescript-5.6+-blue.svg)
![Docker](https://img.shields.io/badge/docker-ready-green.svg)

A modern, production-ready cryptocurrency exchange platform built with Rust backend and React frontend, featuring real-time trading, comprehensive portfolio management, and enterprise-grade observability.

## Features

### Trading & Market Data
- **Real-time Trading**: Live order execution with WebSocket connections
- **Order Types**: Market orders, limit orders, stop-loss, and take-profit
- **Order Book**: Real-time bid/ask spreads with depth visualization
- **Price Charts**: Interactive candlestick charts with technical indicators
- **Market Data**: Live price feeds and 24h statistics

### Portfolio Management
- **Multi-Currency Wallets**: Support for multiple cryptocurrencies
- **Portfolio Analytics**: Real-time P&L tracking and performance metrics
- **Transaction History**: Comprehensive audit trail for all activities
- **Balance Management**: Available and locked balance tracking
- **Asset Allocation**: Visual portfolio distribution charts

### Security & Authentication
- **JWT Authentication**: Secure token-based authentication
- **Two-Factor Authentication (2FA)**: TOTP-based additional security layer
- **Password Security**: bcrypt hashing with configurable cost
- **Session Management**: Redis-based session storage
- **Rate Limiting**: Request throttling and DDoS protection

### User Experience
- **Responsive Design**: Mobile-first responsive interface
- **Dark/Light Theme**: User preference-based theme switching
- **Real-time Notifications**: Instant trade and system notifications
- **Progressive Web App**: PWA support for mobile installation
- **Accessibility**: WCAG 2.1 AA compliant interface

## Architecture

### System Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Backend API   │    │   Database      │
│   (React)       │◄──►│   (Rust)        │◄──►│   (PostgreSQL)  │
│   Port: 3001    │    │   Port: 8080    │    │   Port: 5432    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │              ┌─────────────────┐              │
         │              │     Redis       │              │
         └──────────────►│   (Cache)       │◄─────────────┘
                        │   Port: 6379    │
                        └─────────────────┘
```

### Technology Stack

#### Backend
- **Language**: Rust 1.82+
- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL 15 with SQLx
- **Caching**: Redis 7
- **Authentication**: JWT with bcrypt
- **WebSockets**: Native Tokio WebSocket support
- **Configuration**: Environment-based configuration

#### Frontend
- **Framework**: React 18 with TypeScript 5.6+
- **Build Tool**: Vite 6.0+
- **Styling**: Tailwind CSS 3.4+
- **State Management**: Zustand 4.4+
- **Data Fetching**: TanStack Query 5.51+
- **Routing**: React Router 6.26+
- **Forms**: React Hook Form 7.52+ with Zod validation
- **Charts**: Recharts 2.12+ and Lightweight Charts 4.1+

#### Infrastructure
- **Containerization**: Docker & Docker Compose
- **Orchestration**: Kubernetes with Helm charts
- **Web Server**: Nginx (reverse proxy & static files)
- **Monitoring**: Prometheus + Grafana
- **Logging**: Loki + Promtail
- **Tracing**: OpenTelemetry + Jaeger
- **Secrets**: HashiCorp Vault
- **CI/CD**: GitHub Actions

##  Quick Start

### Prerequisites

- **Docker & Docker Compose**: Latest version
- **Node.js**: 18+ (for local development)
- **Rust**: 1.82+ (for local development)
- **PostgreSQL**: 15+ (for local development)
- **Redis**: 7+ (for local development)

### Option 1: Docker Compose (Recommended)

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/cryptotrade-exchange.git
   cd cryptotrade-exchange
   ```

2. **Start all services**
   ```bash
   docker-compose up --build -d
   ```

3. **Access the application**
   - Frontend: http://localhost:3001
   - Backend API: http://localhost:8080
   - Database: localhost:5432
   - Redis: localhost:6379

4. **View logs** (optional)
   ```bash
   docker-compose logs -f
   ```

### Option 2: Local Development

1. **Setup Database**
   ```bash
   # Start PostgreSQL and Redis
   docker run -d --name postgres -p 5432:5432 -e POSTGRES_PASSWORD=postgres123 postgres:15
   docker run -d --name redis -p 6379:6379 redis:7-alpine
   ```

2. **Backend Setup**
   ```bash
   cd backend
   # Copy environment variables
   cp .env.example .env
   
   # Install dependencies and run migrations
   cargo install sqlx-cli
   sqlx migrate run
   
   # Start the backend
   cargo run --bin cryptotrade-api
   ```

3. **Frontend Setup**
   ```bash
   cd frontend
   # Install dependencies
   npm install
   
   # Start development server
   npm run dev
   ```

## Project Structure

```
cryptotrade-exchange/
├── backend/                    # Rust backend application
│   ├── api/                   # HTTP handlers and WebSocket
│   ├── core/                  # Business logic and models
│   └── migrations/            # Database migrations
├── frontend/                  # React frontend application
│   ├── src/
│   │   ├── components/        # Reusable UI components
│   │   ├── pages/            # Page-level components
│   │   ├── services/         # API clients and utilities
│   │   ├── stores/           # State management
│   │   └── types/            # TypeScript type definitions
│   ├── public/               # Static assets
│   └── Dockerfile            # Frontend container
├── k8s/                      # Kubernetes deployment manifests
│   ├── base/                 # Base Kubernetes resources
│   ├── helm/                 # Helm charts
│   └── overlays/             # Environment-specific overlays
├── monitoring/               # Observability configuration
│   ├── grafana/              # Grafana dashboards and config
│   ├── prometheus/           # Prometheus configuration
│   ├── loki/                 # Loki configuration
│   └── otel/                 # OpenTelemetry collector config
├── docker-compose.yml        # Development environment
├── DEPLOYMENT.md             # Deployment instructions
└── README.md                 # This file
```

## Configuration

### Environment Variables

#### Backend (.env)
```bash
# Database
DATABASE_URL=postgresql://postgres:postgres123@localhost:5432/cryptotrade_db
DB_MAX_CONNECTIONS=10

# Redis
REDIS_URL=redis://:redis123@localhost:6379
REDIS_MAX_CONNECTIONS=10

# Security
JWT_SECRET=your-super-secret-jwt-key
BCRYPT_COST=12

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
CORS_ALLOWED_ORIGINS=http://localhost:3001

# Rate Limiting
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW_SECONDS=60
```

#### Frontend (.env)
```bash
# API Configuration
VITE_API_URL=http://localhost:8080
VITE_WS_URL=ws://localhost:8080/ws

# Application
VITE_APP_NAME=CryptoTrade Exchange
VITE_APP_VERSION=1.0.0
```

## Database Schema

### Core Entities

```sql
-- Users and Authentication
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    two_factor_secret VARCHAR(255),
    is_two_factor_enabled BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Multi-currency Accounts
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    currency VARCHAR(10) NOT NULL,
    balance DECIMAL(20,8) DEFAULT 0,
    available_balance DECIMAL(20,8) DEFAULT 0,
    locked_balance DECIMAL(20,8) DEFAULT 0
);

-- Trading Pairs
CREATE TABLE trading_pairs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol VARCHAR(20) UNIQUE NOT NULL,
    base_currency VARCHAR(10) NOT NULL,
    quote_currency VARCHAR(10) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE
);

-- Orders
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    trading_pair_id UUID NOT NULL REFERENCES trading_pairs(id),
    order_type VARCHAR(20) NOT NULL, -- 'market', 'limit'
    side VARCHAR(10) NOT NULL, -- 'buy', 'sell'
    quantity DECIMAL(20,8) NOT NULL,
    price DECIMAL(20,8), -- NULL for market orders
    status VARCHAR(20) DEFAULT 'pending'
);
```

##  API Documentation

### Authentication Endpoints

```http
POST /api/v1/auth/register
POST /api/v1/auth/login
POST /api/v1/auth/refresh
POST /api/v1/auth/logout
```

### Trading Endpoints

```http
GET  /api/v1/trading-pairs          # Get all trading pairs
GET  /api/v1/market-data            # Get market data
GET  /api/v1/order-book/{pair_id}   # Get order book
POST /api/v1/orders                 # Create order
GET  /api/v1/orders                 # Get user orders
DELETE /api/v1/orders/{order_id}    # Cancel order
```

### Portfolio Endpoints

```http
GET /api/v1/portfolio               # Get portfolio summary
GET /api/v1/portfolio/history       # Get portfolio history
GET /api/v1/accounts                # Get user accounts
GET /api/v1/transactions            # Get transaction history
```

### WebSocket Events

```javascript
// Market data subscription
{
  "action": "subscribe",
  "channel": "market_data",
  "symbol": "BTC/USD"
}

// Order book subscription
{
  "action": "subscribe",
  "channel": "orderbook",
  "symbol": "BTC/USD"
}

// Portfolio updates
{
  "action": "subscribe",
  "channel": "portfolio"
}
```

##  Monitoring & Observability

### Accessing Monitoring Tools

When running with Docker Compose, access these monitoring interfaces:

- **Grafana**: http://localhost:3000 (admin/admin123)
- **Prometheus**: http://localhost:9090
- **Jaeger**: http://localhost:16686
- **HashiCorp Vault**: http://localhost:8200

### Key Metrics

- **Application Performance**: Request latency, throughput, error rates
- **Business Metrics**: Trading volume, active users, order execution times
- **Infrastructure**: CPU, memory, disk usage, network I/O
- **Security**: Failed login attempts, suspicious activities

### Alerts

Configure alerts for:
- High error rates (>5%)
- High response times (>500ms)
- Database connection issues
- Memory usage (>80%)
- Failed authentication attempts

## Testing

### Running Tests

```bash
# Backend tests
cd backend
cargo test

# Frontend tests
cd frontend
npm test

# Integration tests
npm run test:e2e
```

### Test Coverage

- **Backend**: Unit tests for business logic, integration tests for API
- **Frontend**: Component tests with React Testing Library
- **End-to-End**: Cypress tests for critical user journeys

## Deployment

### Production Deployment

#### Option 1: Kubernetes (Recommended)

```bash
# Deploy with Helm
helm install cryptotrade-exchange ./k8s/helm/cryptotrade-exchange \
  --namespace production \
  --values ./k8s/helm/cryptotrade-exchange/values-prod.yaml
```

#### Option 2: Docker Swarm

```bash
# Deploy to swarm
docker stack deploy -c docker-compose.prod.yml cryptotrade
```

### Environment-Specific Configurations

- **Development**: `docker-compose.yml`
- **Staging**: `k8s/overlays/staging/`
- **Production**: `k8s/overlays/prod/`

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed deployment instructions.

##  Security

### Security Features

- **Authentication**: JWT tokens with refresh mechanism
- **Authorization**: Role-based access control
- **Password Security**: bcrypt with configurable cost
- **Two-Factor Authentication**: TOTP support
- **Rate Limiting**: Request throttling
- **Input Validation**: Comprehensive request validation
- **SQL Injection Protection**: Compile-time checked queries
- **XSS Protection**: Content Security Policy headers

### Security Best Practices

1. **Use HTTPS in production**
2. **Configure proper CORS policies**
3. **Set up rate limiting**
4. **Enable 2FA for all users**
5. **Regular security audits**
6. **Keep dependencies updated**

##  Contributing

We welcome contributions! Please see our contributing guidelines:

### Development Workflow

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes**
4. **Run tests**
   ```bash
   cargo test && npm test
   ```
5. **Commit your changes**
   ```bash
   git commit -m 'Add amazing feature'
   ```
6. **Push to your branch**
   ```bash
   git push origin feature/amazing-feature
   ```
7. **Open a Pull Request**

### Code Standards

- **Rust**: Follow `rustfmt` and `clippy` recommendations
- **TypeScript**: Use ESLint and Prettier configurations
- **Commits**: Use conventional commit messages
- **Documentation**: Update README and inline documentation

## 📈 Performance

### Benchmarks

- **API Response Time**: <100ms average
- **WebSocket Latency**: <50ms
- **Database Query Time**: <10ms average
- **Frontend Bundle Size**: <500KB gzipped
- **Lighthouse Score**: 90+

### Optimization Strategies

- **Backend**: Connection pooling, caching, async processing
- **Frontend**: Code splitting, lazy loading, service workers
- **Database**: Proper indexing, query optimization
- **Infrastructure**: CDN, load balancing, horizontal scaling

##  Troubleshooting

### Common Issues

#### Docker Issues
```bash
# Clean Docker environment
docker-compose down --volumes --remove-orphans
docker system prune -f

# Rebuild containers
docker-compose up --build --force-recreate
```

#### Database Connection Issues
```bash
# Check PostgreSQL connection
docker-compose exec postgres psql -U postgres -d cryptotrade_db

# Run migrations
docker-compose exec backend sqlx migrate run
```

#### Frontend Build Issues
```bash
# Clear node modules and reinstall
rm -rf node_modules package-lock.json
npm install

# Clear Vite cache
rm -rf node_modules/.vite
```

### Getting Help

- **Issues**: Report bugs on GitHub Issues
- **Discussions**: Join GitHub Discussions
- **Documentation**: Check the wiki for detailed guides
- **Security**: Report security issues privately

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Rust Community**: For the amazing Rust ecosystem
- **React Team**: For the excellent React framework
- **Open Source Contributors**: All the amazing library maintainers
- **Security Researchers**: For keeping the ecosystem secure



---

**Built  using Rust, React, and modern DevOps practices.**

For more detailed information, see our [technical blog post](blog.md) about building this project.
