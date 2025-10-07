// Core types for the CryptoTrade Exchange frontend
export interface User {
  id: string;
  email: string;
  username: string;
  firstName?: string;
  lastName?: string;
  isVerified: boolean;
  twoFaEnabled: boolean;
  kycStatus: 'pending' | 'approved' | 'rejected' | 'required';
}

export interface AuthResponse {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
  user: User;
}

export interface Account {
  id: string;
  userId: string;
  currency: string;
  balance: number;
  availableBalance: number;
  lockedBalance: number;
  createdAt: string;
  updatedAt: string;
}

export interface TradingPair {
  id: string;
  symbol: string;
  baseCurrency: string;
  quoteCurrency: string;
  isActive: boolean;
  minOrderSize: number;
  maxOrderSize: number;
  pricePrecision: number;
  quantityPrecision: number;
  makerFee: number;
  takerFee: number;
  createdAt: string;
}

export interface Order {
  id: string;
  userId: string;
  tradingPairId: string;
  orderType: OrderType;
  side: OrderSide;
  quantity: number;
  price?: number;
  filledQuantity: number;
  remainingQuantity: number;
  status: OrderStatus;
  timeInForce: TimeInForce;
  stopPrice?: number;
  createdAt: string;
  updatedAt: string;
  expiresAt?: string;
}

export type OrderType = 'market' | 'limit' | 'stop_loss' | 'take_profit' | 'stop_loss_limit' | 'take_profit_limit';
export type OrderSide = 'buy' | 'sell';
export type OrderStatus = 'pending' | 'open' | 'partially_filled' | 'filled' | 'cancelled' | 'rejected' | 'expired';
export type TimeInForce = 'gtc' | 'ioc' | 'fok' | 'gtd';

export interface CreateOrderRequest {
  tradingPairId: string;
  orderType: OrderType;
  side: OrderSide;
  quantity: number;
  price?: number;
  timeInForce?: TimeInForce;
  stopPrice?: number;
}

export interface Trade {
  id: string;
  tradingPairId: string;
  buyerOrderId: string;
  sellerOrderId: string;
  buyerUserId: string;
  sellerUserId: string;
  price: number;
  quantity: number;
  buyerFee: number;
  sellerFee: number;
  createdAt: string;
}

export interface MarketData {
  tradingPairId: string;
  symbol: string;
  lastPrice: number;
  volume24h: number;
  high24h: number;
  low24h: number;
  priceChange24h: number;
  priceChangePercent24h: number;
  bidPrice?: number;
  askPrice?: number;
  updatedAt: string;
}

export interface OrderBook {
  tradingPairId: string;
  symbol: string;
  bids: OrderBookLevel[];
  asks: OrderBookLevel[];
  timestamp: string;
}

export interface OrderBookLevel {
  price: number;
  quantity: number;
  count: number;
}

export interface Candlestick {
  timestamp: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

export interface Portfolio {
  userId: string;
  totalValueUsd: number;
  accounts: AccountBalance[];
  performance24h: PerformanceMetrics;
  openOrdersCount: number;
  totalTrades: number;
}

export interface AccountBalance {
  currency: string;
  balance: number;
  availableBalance: number;
  lockedBalance: number;
  usdValue: number;
  percentage: number;
}

export interface PerformanceMetrics {
  pnl24h: number;
  pnlPercentage24h: number;
  totalVolume24h: number;
  totalFees24h: number;
}

export interface PortfolioSnapshot {
  userId: string;
  totalValueUsd: number;
  snapshotDate: string;
  createdAt: string;
}

export interface WebSocketMessage {
  type: string;
  channel?: string;
  symbol?: string;
  data: any;
  timestamp: string;
}

export interface Transaction {
  id: string;
  userId: string;
  transactionType: 'deposit' | 'withdrawal' | 'trade' | 'fee';
  currency: string;
  amount: number;
  fee: number;
  status: 'pending' | 'confirmed' | 'failed' | 'cancelled';
  externalId?: string;
  address?: string;
  confirmations?: number;
  requiredConfirmations?: number;
  createdAt: string;
  updatedAt: string;
}

export interface DepositAddress {
  id: string;
  userId: string;
  currency: string;
  address: string;
  tag?: string;
  network: string;
  isActive: boolean;
  createdAt: string;
}

// API Request/Response types
export interface LoginRequest {
  email: string;
  password: string;
  totpCode?: string;
}

export interface RegisterRequest {
  email: string;
  username: string;
  password: string;
  firstName?: string;
  lastName?: string;
}

export interface RefreshTokenRequest {
  refreshToken: string;
}

export interface TokenResponse {
  accessToken: string;
  expiresIn: number;
}

export interface TwoFactorResponse {
  qrCodeUrl: string;
  secret: string;
}

export interface ConfirmTwoFactorRequest {
  totpCode: string;
}

export interface ApiError {
  error: string;
  message: string;
}

export interface SuccessResponse {
  message: string;
}

// Chart data types
export interface ChartData {
  time: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

export interface PriceTickerData {
  symbol: string;
  price: number;
  change: number;
  changePercent: number;
  volume: number;
  high: number;
  low: number;
}

// UI State types
export interface AppState {
  user: User | null;
  isAuthenticated: boolean;
  theme: 'light' | 'dark';
  selectedTradingPair: TradingPair | null;
  portfolio: Portfolio | null;
}

export interface TradingState {
  orderBook: OrderBook | null;
  recentTrades: Trade[];
  marketData: MarketData | null;
  userOrders: Order[];
  candlestickData: Candlestick[];
  selectedInterval: string;
}

export interface UIState {
  sidebarOpen: boolean;
  loading: boolean;
  notifications: Notification[];
}

export interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message: string;
  timestamp: string;
  read: boolean;
}
