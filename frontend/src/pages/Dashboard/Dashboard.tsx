import React from 'react';
import { useQuery } from '@tanstack/react-query';
import {
  ChartBarIcon,
  CurrencyDollarIcon,
  ArrowTrendingUpIcon,
  ArrowTrendingDownIcon,
} from '@heroicons/react/24/outline';
import { apiClient } from '../../services/api';
import { useAuthStore } from '../../stores';

const Dashboard: React.FC = () => {
  const { user } = useAuthStore();

  const { data: portfolio } = useQuery({
    queryKey: ['portfolio'],
    queryFn: () => apiClient.getPortfolio(),
  });

  const { data: marketData } = useQuery({
    queryKey: ['marketData'],
    queryFn: () => apiClient.getAllMarketData(),
    refetchInterval: 5000, // Refresh every 5 seconds
  });

  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(value);
  };

  const formatPercentage = (value: number) => {
    return `${value >= 0 ? '+' : ''}${value.toFixed(2)}%`;
  };

  return (
    <div className="p-6">
      {/* Welcome Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Welcome back, {user?.username}!
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Here's what's happening with your portfolio today.
        </p>
      </div>

      {/* Portfolio Overview */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <CurrencyDollarIcon className="h-8 w-8 text-primary-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500 dark:text-gray-400">
                Total Portfolio Value
              </p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {portfolio ? formatCurrency(portfolio.totalValueUsd) : '$0.00'}
              </p>
            </div>
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <ArrowTrendingUpIcon className="h-8 w-8 text-success-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500 dark:text-gray-400">
                24h P&L
              </p>
              <p className={`text-2xl font-bold ${
                  (portfolio?.performance24h?.pnl24h ?? 0) >= 0 ? 'text-success-600' : 'text-error-600'
                }`}>
                {portfolio ? formatCurrency(portfolio.performance24h?.pnl24h ?? 0) : '$0.00'}
              </p>
            </div>
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <ChartBarIcon className="h-8 w-8 text-primary-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500 dark:text-gray-400">
                24h Volume
              </p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {portfolio ? formatCurrency(portfolio.performance24h?.totalVolume24h ?? 0) : '$0.00'}
              </p>
            </div>
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <ArrowTrendingDownIcon className="h-8 w-8 text-gray-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-500 dark:text-gray-400">
                Open Orders
              </p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {portfolio?.openOrdersCount || 0}
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Market Overview */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow">
          <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
              Market Overview
            </h2>
          </div>
          <div className="p-6">
            <div className="space-y-4">
              {marketData?.slice(0, 5).map((market) => (
                <div key={market.tradingPairId} className="flex items-center justify-between">
                  <div>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {market.symbol}
                    </p>
                    <p className="text-sm text-gray-500 dark:text-gray-400">
                      Vol: {formatCurrency(market.volume24h)}
                    </p>
                  </div>
                  <div className="text-right">
                    <p className="font-medium text-gray-900 dark:text-white">
                      {formatCurrency(market.lastPrice)}
                    </p>
                    <p className={`text-sm ${
                      market.priceChangePercent24h >= 0 ? 'text-success-600' : 'text-danger-600'
                    }`}>
                      {formatPercentage(market.priceChangePercent24h)}
                    </p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow">
          <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
              Portfolio Allocation
            </h2>
          </div>
          <div className="p-6">
            <div className="space-y-4">
              {portfolio?.accounts.slice(0, 5).map((account) => (
                <div key={account.currency} className="flex items-center justify-between">
                  <div>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {account.currency}
                    </p>
                    <p className="text-sm text-gray-500 dark:text-gray-400">
                      {account.balance.toFixed(8)}
                    </p>
                  </div>
                  <div className="text-right">
                    <p className="font-medium text-gray-900 dark:text-white">
                      {formatCurrency(account.usdValue)}
                    </p>
                    <p className="text-sm text-gray-500 dark:text-gray-400">
                      {account.percentage.toFixed(1)}%
                    </p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Quick Actions */}
      <div className="mt-8">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Quick Actions
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <button className="p-4 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors">
            Start Trading
          </button>
          <button className="p-4 bg-success-600 text-white rounded-lg hover:bg-success-700 transition-colors">
            Deposit Funds
          </button>
          <button className="p-4 bg-gray-600 text-white rounded-lg hover:bg-gray-700 transition-colors">
            View Portfolio
          </button>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
