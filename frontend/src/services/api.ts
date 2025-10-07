import axios, { type AxiosInstance } from 'axios';
import type {
  AuthResponse,
  LoginRequest,
  RegisterRequest,
  RefreshTokenRequest,
  TokenResponse,
  User,
  Account,
  Order,
  CreateOrderRequest,
  Trade,
  MarketData,
  OrderBook,
  Candlestick,
  Portfolio,
  PortfolioSnapshot,
  TradingPair,
  Transaction,
  DepositAddress,
  TwoFactorResponse,
  ConfirmTwoFactorRequest,
  SuccessResponse,
} from '../types';

class ApiClient {
  private client: AxiosInstance;
  private baseURL: string;

  constructor() {
    this.baseURL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

    this.client = axios.create({
      baseURL: this.baseURL,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Request interceptor to add auth token
    this.client.interceptors.request.use(
      (config) => {
        const token = localStorage.getItem('accessToken');
        if (token) {
          config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
      },
      (error) => Promise.reject(error)
    );

    // Response interceptor to handle token refresh
    this.client.interceptors.response.use(
      (response) => response,
      async (error) => {
        const originalRequest = error.config;

        if (error.response?.status === 401 && !originalRequest._retry) {
          originalRequest._retry = true;

          try {
            const refreshToken = localStorage.getItem('refreshToken');
            if (refreshToken) {
              const response = await this.refreshToken({ refreshToken });
              localStorage.setItem('accessToken', response.accessToken);
              originalRequest.headers.Authorization = `Bearer ${response.accessToken}`;
              return this.client(originalRequest);
            }
          } catch (refreshError) {
            // Refresh failed, redirect to login
            localStorage.removeItem('accessToken');
            localStorage.removeItem('refreshToken');
            window.location.href = '/login';
          }
        }

        return Promise.reject(error);
      }
    );
  }

  // Authentication endpoints
  async login(data: LoginRequest): Promise<AuthResponse> {
    const response = await this.client.post<AuthResponse>('/api/v1/auth/login', data);
    return response.data;
  }

  async register(data: RegisterRequest): Promise<AuthResponse> {
    const response = await this.client.post<AuthResponse>('/api/v1/auth/register', data);
    return response.data;
  }

  async refreshToken(data: RefreshTokenRequest): Promise<TokenResponse> {
    const response = await this.client.post<TokenResponse>('/api/v1/auth/refresh', data);
    return response.data;
  }

  async logout(): Promise<void> {
    localStorage.removeItem('accessToken');
    localStorage.removeItem('refreshToken');
  }

  // User endpoints
  async getUserProfile(): Promise<User> {
    const response = await this.client.get<User>('/api/v1/user/profile');
    return response.data;
  }

  async getUserAccounts(): Promise<Account[]> {
    const response = await this.client.get<Account[]>('/api/v1/user/accounts');
    return response.data;
  }

  async enable2FA(): Promise<TwoFactorResponse> {
    const response = await this.client.post<TwoFactorResponse>('/api/v1/user/2fa/enable');
    return response.data;
  }

  async confirm2FA(data: ConfirmTwoFactorRequest): Promise<SuccessResponse> {
    const response = await this.client.post<SuccessResponse>('/api/v1/user/2fa/confirm', data);
    return response.data;
  }

  async disable2FA(data: ConfirmTwoFactorRequest): Promise<SuccessResponse> {
    const response = await this.client.post<SuccessResponse>('/api/v1/user/2fa/disable', data);
    return response.data;
  }

  // Trading endpoints
  async createOrder(data: CreateOrderRequest): Promise<Order> {
    const response = await this.client.post<Order>('/api/v1/orders', data);
    return response.data;
  }

  async getUserOrders(status?: string, limit?: number): Promise<Order[]> {
    const params = new URLSearchParams();
    if (status) params.append('status', status);
    if (limit) params.append('limit', limit.toString());

    const response = await this.client.get<Order[]>(`/api/v1/orders?${params}`);
    return response.data;
  }

  async cancelOrder(orderId: string): Promise<Order> {
    const response = await this.client.delete<Order>(`/api/v1/orders/${orderId}`);
    return response.data;
  }

  async getUserTrades(limit?: number): Promise<Trade[]> {
    const params = new URLSearchParams();
    if (limit) params.append('limit', limit.toString());

    const response = await this.client.get<Trade[]>(`/api/v1/trades?${params}`);
    return response.data;
  }

  // Market data endpoints
  async getAllMarketData(): Promise<MarketData[]> {
    const response = await this.client.get<MarketData[]>('/api/v1/market-data');
    return response.data;
  }

  async getMarketData(pairId: string): Promise<MarketData> {
    const response = await this.client.get<MarketData>(`/api/v1/market-data/${pairId}`);
    return response.data;
  }

  async getOrderBook(pairId: string, depth?: number): Promise<OrderBook> {
    const params = new URLSearchParams();
    if (depth) params.append('depth', depth.toString());

    const response = await this.client.get<OrderBook>(`/api/v1/order-book/${pairId}?${params}`);
    return response.data;
  }

  async getRecentTrades(pairId: string, limit?: number): Promise<Trade[]> {
    const params = new URLSearchParams();
    if (limit) params.append('limit', limit.toString());

    const response = await this.client.get<Trade[]>(`/api/v1/trades/${pairId}?${params}`);
    return response.data;
  }

  async getCandlestickData(
    pairId: string,
    interval?: string,
    limit?: number
  ): Promise<Candlestick[]> {
    const params = new URLSearchParams();
    if (interval) params.append('interval', interval);
    if (limit) params.append('limit', limit.toString());

    const response = await this.client.get<Candlestick[]>(`/api/v1/candlesticks/${pairId}?${params}`);
    return response.data;
  }

  // Portfolio endpoints
  async getPortfolio(): Promise<Portfolio> {
    const response = await this.client.get<Portfolio>('/api/v1/portfolio');
    return response.data;
  }

  async getPortfolioHistory(period?: string): Promise<PortfolioSnapshot[]> {
    const params = new URLSearchParams();
    if (period) params.append('period', period);

    const response = await this.client.get<PortfolioSnapshot[]>(`/api/v1/portfolio/history?${params}`);
    return response.data;
  }

  // Trading pairs endpoints
  async getTradingPairs(): Promise<TradingPair[]> {
    const response = await this.client.get<TradingPair[]>('/api/v1/trading-pairs');
    return response.data;
  }

  // Transaction endpoints
  async getTransactions(type?: string, limit?: number): Promise<Transaction[]> {
    const params = new URLSearchParams();
    if (type) params.append('type', type);
    if (limit) params.append('limit', limit.toString());

    const response = await this.client.get<Transaction[]>(`/api/v1/transactions?${params}`);
    return response.data;
  }

  async getDepositAddresses(): Promise<DepositAddress[]> {
    const response = await this.client.get<DepositAddress[]>('/api/v1/deposit-addresses');
    return response.data;
  }

  async generateDepositAddress(currency: string, network: string): Promise<DepositAddress> {
    const response = await this.client.post<DepositAddress>('/api/v1/deposit-addresses', {
      currency,
      network,
    });
    return response.data;
  }

  // Health check
  async healthCheck(): Promise<{ status: string; timestamp: string; version: string }> {
    const response = await this.client.get('/api/v1/health');
    return response.data;
  }
}

// WebSocket client for real-time data
export class WebSocketClient {
  private ws: WebSocket | null = null;
  private url: string;
  private subscriptions: Set<string> = new Set();
  private eventHandlers: Map<string, Array<(data: any) => void>> = new Map();
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectInterval = 1000;

  constructor() {
    this.url = import.meta.env.VITE_WS_URL || 'ws://localhost:8080/ws';
  }

  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(this.url);

        this.ws.onopen = () => {
          console.log('WebSocket connected');
          this.reconnectAttempts = 0;
          resolve();
        };

        this.ws.onmessage = (event) => {
          try {
            const message = JSON.parse(event.data);
            this.handleMessage(message);
          } catch (error) {
            console.error('Failed to parse WebSocket message:', error);
          }
        };

        this.ws.onclose = () => {
          console.log('WebSocket disconnected');
          this.reconnect();
        };

        this.ws.onerror = (error) => {
          console.error('WebSocket error:', error);
          reject(error);
        };
      } catch (error) {
        reject(error);
      }
    });
  }

  private reconnect(): void {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      console.log(`Attempting to reconnect... (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);

      setTimeout(() => {
        this.connect().catch(() => {
          // Reconnection failed, will try again
        });
      }, this.reconnectInterval * this.reconnectAttempts);
    }
  }

  subscribe(channel: string, symbol?: string): void {
    const subscription = symbol ? `${channel}:${symbol}` : channel;

    if (!this.subscriptions.has(subscription)) {
      this.subscriptions.add(subscription);

      if (this.ws?.readyState === WebSocket.OPEN) {
        this.ws.send(JSON.stringify({
          action: 'subscribe',
          channel,
          symbol,
        }));
      }
    }
  }

  unsubscribe(channel: string, symbol?: string): void {
    const subscription = symbol ? `${channel}:${symbol}` : channel;

    if (this.subscriptions.has(subscription)) {
      this.subscriptions.delete(subscription);

      if (this.ws?.readyState === WebSocket.OPEN) {
        this.ws.send(JSON.stringify({
          action: 'unsubscribe',
          channel,
          symbol,
        }));
      }
    }
  }

  on(event: string, handler: (data: any) => void): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, []);
    }
    this.eventHandlers.get(event)!.push(handler);
  }

  off(event: string, handler: (data: any) => void): void {
    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      const index = handlers.indexOf(handler);
      if (index > -1) {
        handlers.splice(index, 1);
      }
    }
  }

  private handleMessage(message: any): void {
    const handlers = this.eventHandlers.get(message.type);
    if (handlers) {
      handlers.forEach(handler => handler(message));
    }
  }

  disconnect(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.subscriptions.clear();
    this.eventHandlers.clear();
  }

  ping(): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ action: 'ping' }));
    }
  }
}

// Export singleton instances
export const apiClient = new ApiClient();
export const wsClient = new WebSocketClient();
