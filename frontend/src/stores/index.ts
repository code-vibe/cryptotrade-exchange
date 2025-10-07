import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type {
  User,
  TradingState,
  UIState,
  Notification,
  TradingPair,
  Portfolio,
  Order,
  Trade,
  MarketData,
  OrderBook,
  Candlestick,
} from '../types';

// Auth Store
interface AuthStore {
  user: User | null;
  isAuthenticated: boolean;
  accessToken: string | null;
  refreshToken: string | null;
  login: (user: User, accessToken: string, refreshToken: string) => void;
  logout: () => void;
  updateUser: (user: Partial<User>) => void;
}

export const useAuthStore = create<AuthStore>()(
  persist(
    (set, get) => ({
      user: null,
      isAuthenticated: false,
      accessToken: null,
      refreshToken: null,

      login: (user, accessToken, refreshToken) => {
        localStorage.setItem('accessToken', accessToken);
        localStorage.setItem('refreshToken', refreshToken);
        set({
          user,
          isAuthenticated: true,
          accessToken,
          refreshToken,
        });
      },

      logout: () => {
        localStorage.removeItem('accessToken');
        localStorage.removeItem('refreshToken');
        set({
          user: null,
          isAuthenticated: false,
          accessToken: null,
          refreshToken: null,
        });
      },

      updateUser: (userData) => {
        const currentUser = get().user;
        if (currentUser) {
          set({
            user: { ...currentUser, ...userData },
          });
        }
      },
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({
        user: state.user,
        isAuthenticated: state.isAuthenticated,
        accessToken: state.accessToken,
        refreshToken: state.refreshToken,
      }),
    }
  )
);

// Trading Store
interface TradingStore extends TradingState {
  selectedTradingPair: TradingPair | null;
  orderType: 'market' | 'limit';
  orderSide: 'buy' | 'sell';
  orderQuantity: string;
  orderPrice: string;

  setSelectedTradingPair: (pair: TradingPair | null) => void;
  setOrderBook: (orderBook: OrderBook) => void;
  setRecentTrades: (trades: Trade[]) => void;
  addRecentTrade: (trade: Trade) => void;
  setMarketData: (data: MarketData) => void;
  setUserOrders: (orders: Order[]) => void;
  updateOrder: (order: Order) => void;
  removeOrder: (orderId: string) => void;
  setCandlestickData: (data: Candlestick[]) => void;
  setSelectedInterval: (interval: string) => void;
  setOrderType: (type: 'market' | 'limit') => void;
  setOrderSide: (side: 'buy' | 'sell') => void;
  setOrderQuantity: (quantity: string) => void;
  setOrderPrice: (price: string) => void;
  resetOrderForm: () => void;
}

export const useTradingStore = create<TradingStore>((set, get) => ({
  selectedTradingPair: null,
  orderBook: null,
  recentTrades: [],
  marketData: null,
  userOrders: [],
  candlestickData: [],
  selectedInterval: '1h',
  orderType: 'limit',
  orderSide: 'buy',
  orderQuantity: '',
  orderPrice: '',

  setSelectedTradingPair: (pair) => set({ selectedTradingPair: pair }),

  setOrderBook: (orderBook) => set({ orderBook }),

  setRecentTrades: (trades) => set({ recentTrades: trades }),

  addRecentTrade: (trade) => {
    const currentTrades = get().recentTrades;
    const newTrades = [trade, ...currentTrades].slice(0, 100); // Keep last 100 trades
    set({ recentTrades: newTrades });
  },

  setMarketData: (data) => set({ marketData: data }),

  setUserOrders: (orders) => set({ userOrders: orders }),

  updateOrder: (updatedOrder) => {
    const currentOrders = get().userOrders;
    const newOrders = currentOrders.map(order =>
      order.id === updatedOrder.id ? updatedOrder : order
    );
    set({ userOrders: newOrders });
  },

  removeOrder: (orderId) => {
    const currentOrders = get().userOrders;
    const newOrders = currentOrders.filter(order => order.id !== orderId);
    set({ userOrders: newOrders });
  },

  setCandlestickData: (data) => set({ candlestickData: data }),

  setSelectedInterval: (interval) => set({ selectedInterval: interval }),

  setOrderType: (type) => set({ orderType: type }),

  setOrderSide: (side) => set({ orderSide: side }),

  setOrderQuantity: (quantity) => set({ orderQuantity: quantity }),

  setOrderPrice: (price) => set({ orderPrice: price }),

  resetOrderForm: () => set({
    orderType: 'limit',
    orderSide: 'buy',
    orderQuantity: '',
    orderPrice: '',
  }),
}));

// Portfolio Store
interface PortfolioStore {
  portfolio: Portfolio | null;
  portfolioHistory: any[];
  loading: boolean;

  setPortfolio: (portfolio: Portfolio) => void;
  setPortfolioHistory: (history: any[]) => void;
  setLoading: (loading: boolean) => void;
}

export const usePortfolioStore = create<PortfolioStore>((set) => ({
  portfolio: null,
  portfolioHistory: [],
  loading: false,

  setPortfolio: (portfolio) => set({ portfolio }),
  setPortfolioHistory: (history) => set({ portfolioHistory: history }),
  setLoading: (loading) => set({ loading }),
}));

// UI Store
interface UIStoreType extends UIState {
  theme: 'light' | 'dark';
  sidebarCollapsed: boolean;
  tradingPairSearch: string;
  activeTab: string;

  toggleTheme: () => void;
  setSidebarOpen: (open: boolean) => void;
  toggleSidebar: () => void;
  setSidebarCollapsed: (collapsed: boolean) => void;
  setTradingPairSearch: (search: string) => void;
  setActiveTab: (tab: string) => void;
  setLoading: (loading: boolean) => void;
  addNotification: (notification: Omit<Notification, 'id' | 'timestamp'>) => void;
  removeNotification: (id: string) => void;
  markNotificationAsRead: (id: string) => void;
  clearNotifications: () => void;
}

export const useUIStore = create<UIStoreType>()(
  persist(
    (set, get) => ({
      theme: 'dark',
      sidebarOpen: true,
      sidebarCollapsed: false,
      loading: false,
      notifications: [],
      tradingPairSearch: '',
      activeTab: 'trading',

      toggleTheme: () => {
        const currentTheme = get().theme;
        const newTheme = currentTheme === 'light' ? 'dark' : 'light';
        set({ theme: newTheme });

        // Update document class for Tailwind dark mode
        if (newTheme === 'dark') {
          document.documentElement.classList.add('dark');
        } else {
          document.documentElement.classList.remove('dark');
        }
      },

      setSidebarOpen: (open) => set({ sidebarOpen: open }),

      toggleSidebar: () => {
        const currentState = get().sidebarOpen;
        set({ sidebarOpen: !currentState });
      },

      setSidebarCollapsed: (collapsed) => set({ sidebarCollapsed: collapsed }),

      setTradingPairSearch: (search) => set({ tradingPairSearch: search }),

      setActiveTab: (tab) => set({ activeTab: tab }),

      setLoading: (loading) => set({ loading }),

      addNotification: (notification) => {
        const newNotification: Notification = {
          ...notification,
          id: Date.now().toString(),
          timestamp: new Date().toISOString(),
          read: false,
        };

        const currentNotifications = get().notifications;
        set({
          notifications: [newNotification, ...currentNotifications].slice(0, 50), // Keep last 50
        });
      },

      removeNotification: (id) => {
        const currentNotifications = get().notifications;
        set({
          notifications: currentNotifications.filter(n => n.id !== id),
        });
      },

      markNotificationAsRead: (id) => {
        const currentNotifications = get().notifications;
        set({
          notifications: currentNotifications.map(n =>
            n.id === id ? { ...n, read: true } : n
          ),
        });
      },

      clearNotifications: () => set({ notifications: [] }),
    }),
    {
      name: 'ui-storage',
      partialize: (state) => ({
        theme: state.theme,
        sidebarCollapsed: state.sidebarCollapsed,
      }),
    }
  )
);

// Market Data Store
interface MarketDataStore {
  allMarketData: MarketData[];
  tradingPairs: TradingPair[];

  setAllMarketData: (data: MarketData[]) => void;
  updateMarketData: (data: MarketData) => void;
  setTradingPairs: (pairs: TradingPair[]) => void;
}

export const useMarketDataStore = create<MarketDataStore>((set, get) => ({
  allMarketData: [],
  tradingPairs: [],

  setAllMarketData: (data) => set({ allMarketData: data }),

  updateMarketData: (updatedData) => {
    const currentData = get().allMarketData;
    const newData = currentData.map(item =>
      item.tradingPairId === updatedData.tradingPairId ? updatedData : item
    );

    // If not found, add it
    if (!currentData.find(item => item.tradingPairId === updatedData.tradingPairId)) {
      newData.push(updatedData);
    }

    set({ allMarketData: newData });
  },

  setTradingPairs: (pairs) => set({ tradingPairs: pairs }),
}));

// Export all stores for easy access
export const useStores = () => ({
  auth: useAuthStore(),
  trading: useTradingStore(),
  portfolio: usePortfolioStore(),
  ui: useUIStore(),
  marketData: useMarketDataStore(),
});
