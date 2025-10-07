-- Create order types
CREATE TYPE order_type AS ENUM ('market', 'limit', 'stop_loss', 'take_profit', 'stop_loss_limit', 'take_profit_limit');
CREATE TYPE order_side AS ENUM ('buy', 'sell');
CREATE TYPE order_status AS ENUM ('pending', 'open', 'partially_filled', 'filled', 'cancelled', 'rejected', 'expired');
CREATE TYPE time_in_force AS ENUM ('gtc', 'ioc', 'fok', 'gtd');

-- Orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    trading_pair_id UUID NOT NULL REFERENCES trading_pairs(id),
    order_type order_type NOT NULL,
    side order_side NOT NULL,
    quantity DECIMAL(20, 8) NOT NULL,
    price DECIMAL(20, 8),
    filled_quantity DECIMAL(20, 8) DEFAULT 0,
    remaining_quantity DECIMAL(20, 8) NOT NULL,
    status order_status DEFAULT 'pending',
    time_in_force time_in_force DEFAULT 'gtc',
    stop_price DECIMAL(20, 8),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ
);

-- Trades table
CREATE TABLE trades (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    trading_pair_id UUID NOT NULL REFERENCES trading_pairs(id),
    buyer_order_id UUID NOT NULL REFERENCES orders(id),
    seller_order_id UUID NOT NULL REFERENCES orders(id),
    buyer_user_id UUID NOT NULL REFERENCES users(id),
    seller_user_id UUID NOT NULL REFERENCES users(id),
    price DECIMAL(20, 8) NOT NULL,
    quantity DECIMAL(20, 8) NOT NULL,
    buyer_fee DECIMAL(20, 8) DEFAULT 0,
    seller_fee DECIMAL(20, 8) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_trading_pair_id ON orders(trading_pair_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_side_price ON orders(side, price) WHERE status = 'open';
CREATE INDEX idx_orders_created_at ON orders(created_at);

CREATE INDEX idx_trades_trading_pair_id ON trades(trading_pair_id);
CREATE INDEX idx_trades_buyer_user_id ON trades(buyer_user_id);
CREATE INDEX idx_trades_seller_user_id ON trades(seller_user_id);
CREATE INDEX idx_trades_created_at ON trades(created_at);
