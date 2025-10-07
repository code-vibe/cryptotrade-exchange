import React from 'react';

const Trading: React.FC = () => {
  return (
    <div className="h-full bg-gray-900 text-white">
      {/* Trading Interface Grid */}
      <div className="trading-grid">
        {/* Header */}
        <div className="col-span-3 trading-panel flex items-center justify-between px-4">
          <div className="flex items-center space-x-4">
            <h1 className="text-xl font-bold">BTC/USD</h1>
            <div className="flex items-center space-x-2">
              <span className="price-up text-2xl font-mono">$50,245.67</span>
              <span className="price-up text-sm">+2.34%</span>
            </div>
          </div>
          <div className="flex items-center space-x-4 text-sm">
            <div>24h High: <span className="font-mono">$51,200.00</span></div>
            <div>24h Low: <span className="font-mono">$49,100.00</span></div>
            <div>24h Volume: <span className="font-mono">2,456 BTC</span></div>
          </div>
        </div>

        {/* Order Book */}
        <div className="trading-panel">
          <div className="p-4 border-b border-gray-700">
            <h2 className="font-semibold">Order Book</h2>
          </div>
          <div className="flex-1 overflow-auto">
            {/* Asks */}
            <div className="space-y-1 p-2">
              {[...Array(10)].map((_, i) => (
                <div key={i} className="order-book-item order-book-ask">
                  <span className="price-down">{(50300 + i * 10).toLocaleString()}</span>
                  <span>{(Math.random() * 2).toFixed(4)}</span>
                  <span>{(Math.random() * 100000).toLocaleString()}</span>
                </div>
              ))}
            </div>

            {/* Spread */}
            <div className="px-4 py-2 text-center bg-gray-800 border-y border-gray-700">
              <span className="text-sm">Spread: $45.50</span>
            </div>

            {/* Bids */}
            <div className="space-y-1 p-2">
              {[...Array(10)].map((_, i) => (
                <div key={i} className="order-book-item order-book-bid">
                  <span className="price-up">{(50250 - i * 10).toLocaleString()}</span>
                  <span>{(Math.random() * 2).toFixed(4)}</span>
                  <span>{(Math.random() * 100000).toLocaleString()}</span>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Chart */}
        <div className="trading-panel">
          <div className="p-4 border-b border-gray-700">
            <div className="flex items-center justify-between">
              <h2 className="font-semibold">Price Chart</h2>
              <div className="flex space-x-2">
                {['1m', '5m', '15m', '1h', '4h', '1d'].map((interval) => (
                  <button
                    key={interval}
                    className="px-2 py-1 text-xs bg-gray-700 hover:bg-gray-600 rounded"
                  >
                    {interval}
                  </button>
                ))}
              </div>
            </div>
          </div>
          <div className="flex-1 chart-container">
            <div className="flex items-center justify-center h-full text-gray-400">
              TradingView Chart Component
              <br />
              <span className="text-sm">(Chart integration placeholder)</span>
            </div>
          </div>
        </div>

        {/* Order Form */}
        <div className="trading-panel">
          <div className="p-4 border-b border-gray-700">
            <div className="flex space-x-2">
              <button className="flex-1 py-2 bg-green-600 hover:bg-green-700 rounded">
                Buy
              </button>
              <button className="flex-1 py-2 bg-red-600 hover:bg-red-700 rounded">
                Sell
              </button>
            </div>
          </div>

          <div className="p-4 space-y-4">
            <div className="flex space-x-2">
              <button className="flex-1 py-1 text-sm bg-gray-700 hover:bg-gray-600 rounded">
                Limit
              </button>
              <button className="flex-1 py-1 text-sm bg-gray-800 hover:bg-gray-700 rounded">
                Market
              </button>
            </div>

            <div>
              <label className="block text-sm mb-1">Price (USD)</label>
              <input
                type="number"
                className="w-full p-2 bg-gray-800 border border-gray-600 rounded"
                placeholder="50,245.67"
              />
            </div>

            <div>
              <label className="block text-sm mb-1">Amount (BTC)</label>
              <input
                type="number"
                className="w-full p-2 bg-gray-800 border border-gray-600 rounded"
                placeholder="0.001"
              />
            </div>

            <div>
              <label className="block text-sm mb-1">Total (USD)</label>
              <input
                type="number"
                className="w-full p-2 bg-gray-800 border border-gray-600 rounded"
                placeholder="50.25"
                readOnly
              />
            </div>

            <button className="w-full py-3 bg-green-600 hover:bg-green-700 rounded font-semibold">
              Buy BTC
            </button>

            <div className="text-xs text-gray-400">
              Available: 1,234.56 USD
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Trading;
