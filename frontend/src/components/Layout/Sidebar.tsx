import React from 'react';
import { NavLink, useLocation } from 'react-router-dom';
import {
  HomeIcon,
  ChartBarIcon,
  BriefcaseIcon,
  ClipboardDocumentListIcon,
  ArrowsRightLeftIcon,
  CogIcon,
  ArrowTrendingUpIcon,
} from '@heroicons/react/24/outline';
import { useUIStore } from '../../stores';

const navigation = [
  { name: 'Dashboard', href: '/', icon: HomeIcon, color: 'text-blue-600' },
  { name: 'Trading', href: '/trading', icon: ChartBarIcon, color: 'text-green-600' },
  { name: 'Portfolio', href: '/portfolio', icon: BriefcaseIcon, color: 'text-purple-600' },
  { name: 'Orders', href: '/orders', icon: ClipboardDocumentListIcon, color: 'text-orange-600' },
  { name: 'Transactions', href: '/transactions', icon: ArrowsRightLeftIcon, color: 'text-indigo-600' },
  { name: 'Settings', href: '/settings', icon: CogIcon, color: 'text-gray-600' },
];

const quickStats = [
  { label: 'BTC', value: '$45,234', change: '+2.3%', positive: true },
  { label: 'ETH', value: '$3,124', change: '-1.2%', positive: false },
  { label: 'Portfolio', value: '$12,456', change: '+5.7%', positive: true },
];

const Sidebar: React.FC = () => {
  const { sidebarOpen } = useUIStore();
  const location = useLocation();

  return (
    <div className={`h-full bg-white border-r border-gray-200 shadow-lg transition-all duration-300 ${
      sidebarOpen ? 'w-64' : 'w-16'
    } flex flex-col`}>
      {/* Logo */}
      <div className="h-16 flex items-center justify-center border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
        {sidebarOpen ? (
          <div className="flex items-center space-x-2">
            <div className="w-8 h-8 bg-gradient-to-br from-blue-600 to-blue-700 rounded-lg flex items-center justify-center shadow-lg">
              <ArrowTrendingUpIcon className="h-5 w-5 text-white" />
            </div>
            <h1 className="text-xl font-bold bg-gradient-to-r from-blue-600 to-blue-800 bg-clip-text text-transparent">
              CryptoTrade
            </h1>
          </div>
        ) : (
          <div className="w-10 h-10 bg-gradient-to-br from-blue-600 to-blue-700 rounded-xl flex items-center justify-center shadow-lg">
            <ArrowTrendingUpIcon className="h-6 w-6 text-white" />
          </div>
        )}
      </div>

      {/* Navigation */}
      <nav className="flex-1 px-3 py-6 space-y-2">
        {navigation.map((item) => {
          const Icon = item.icon;
          const isActive = location.pathname === item.href;

          return (
            <NavLink
              key={item.name}
              to={item.href}
              className={`group flex items-center px-3 py-3 rounded-xl transition-all duration-200 ${
                isActive 
                  ? 'bg-gradient-to-r from-primary-50 to-primary-100 border-l-4 border-primary-500 shadow-sm' 
                  : 'hover:bg-gray-50 hover:shadow-sm'
              }`}
            >
              <Icon
                className={`h-6 w-6 transition-colors duration-200 ${
                  isActive 
                    ? 'text-primary-600' 
                    : `${item.color} group-hover:text-primary-600`
                }`}
              />
              {sidebarOpen && (
                <span className={`ml-3 text-sm font-semibold transition-colors duration-200 ${
                  isActive 
                    ? 'text-primary-900' 
                    : 'text-gray-700 group-hover:text-primary-700'
                }`}>
                  {item.name}
                </span>
              )}
              {isActive && sidebarOpen && (
                <div className="ml-auto w-2 h-2 bg-primary-500 rounded-full"></div>
              )}
            </NavLink>
          );
        })}
      </nav>

      {/* Quick Stats Section */}
      {sidebarOpen && (
        <div className="px-3 py-4 border-t border-gray-200 bg-gradient-to-b from-gray-50 to-white">
          <h3 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-3 text-center">
            Market Overview
          </h3>
          <div className="space-y-2">
            {quickStats.map((stat, index) => (
              <div key={index} className="flex items-center justify-between p-2 rounded-lg bg-white border border-gray-100 hover:shadow-sm transition-shadow">
                <div>
                  <p className="text-xs font-semibold text-gray-800">{stat.label}</p>
                  <p className="text-sm font-bold text-gray-900">{stat.value}</p>
                </div>
                <span className={`text-xs font-semibold px-2 py-1 rounded-full ${
                  stat.positive 
                    ? 'text-green-700 bg-green-100' 
                    : 'text-red-700 bg-red-100'
                }`}>
                  {stat.change}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Upgrade Prompt */}
      {sidebarOpen && (
        <div className="p-3 border-t border-gray-200">
          <div className="bg-gradient-to-br from-blue-500 to-blue-600 rounded-xl p-4 text-white relative overflow-hidden">
            <div className="absolute top-0 right-0 w-20 h-20 bg-white opacity-10 rounded-full -mr-10 -mt-10"></div>
            <div className="relative">
              <h4 className="text-sm font-bold mb-1">Go Premium</h4>
              <p className="text-xs opacity-90 mb-3">Unlock advanced trading features</p>
              <button className="bg-white text-blue-600 px-3 py-1.5 rounded-lg text-xs font-semibold hover:bg-gray-100 transition-colors">
                Upgrade Now
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Sidebar;
