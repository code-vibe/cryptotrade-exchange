import React from 'react';

interface AuthLayoutProps {
  children: React.ReactNode;
}

const AuthLayout: React.FC<AuthLayoutProps> = ({ children }) => {
  return (
    <div className="auth-layout">
      <div className="auth-card fade-in">
        {/* Brand Header */}
        <div className="brand-header">
          <div className="brand-logo">
            <svg className="w-8 h-8 text-white" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
            </svg>
          </div>
          <h1 className="brand-title">
            CryptoTrade Exchange
          </h1>
          <p className="brand-subtitle">
            Professional Cryptocurrency Trading Platform
          </p>
        </div>

        {/* Auth Content */}
        {children}

        {/* Footer */}
        <div className="text-center" style={{ marginTop: '32px' }}>
          <p style={{ fontSize: '14px', color: '#6b7280' }}>
            Secure • Fast • Professional
          </p>
        </div>
      </div>
    </div>
  );
};

export default AuthLayout;
