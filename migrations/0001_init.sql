-- Users and auth (PostgreSQL)
CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  name TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Products
CREATE TABLE IF NOT EXISTS products (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  price_cents BIGINT NOT NULL,
  category TEXT NOT NULL,
  color TEXT,
  size TEXT,
  gender TEXT,
  status TEXT NOT NULL DEFAULT 'active',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Wishlist
CREATE TABLE IF NOT EXISTS wishlist (
  user_id TEXT NOT NULL,
  product_id TEXT NOT NULL,
  added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, product_id),
  CONSTRAINT fk_product FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
);

-- Notification settings
CREATE TABLE IF NOT EXISTS notification_settings (
  user_id TEXT PRIMARY KEY,
  email BOOLEAN NOT NULL DEFAULT TRUE,
  new_arrivals BOOLEAN NOT NULL DEFAULT TRUE,
  promotions BOOLEAN NOT NULL DEFAULT TRUE,
  order_updates BOOLEAN NOT NULL DEFAULT TRUE
);

-- Profiles
CREATE TABLE IF NOT EXISTS user_profiles (
  user_id TEXT PRIMARY KEY,
  name TEXT,
  phone TEXT
);

-- Locale settings
CREATE TABLE IF NOT EXISTS locale_settings (
  user_id TEXT PRIMARY KEY,
  country TEXT,
  language TEXT,
  currency TEXT,
  timezone TEXT
);

-- Static stores
CREATE TABLE IF NOT EXISTS stores (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  address TEXT NOT NULL,
  city TEXT NOT NULL,
  phone TEXT NOT NULL
);

INSERT INTO stores (name, address, city, phone) VALUES
('Thrift HQ', 'Jl. Merdeka No.1', 'Jakarta', '+62 21 0000 1111')
ON CONFLICT DO NOTHING;
INSERT INTO stores (name, address, city, phone) VALUES
('Thrift Bandung', 'Jl. Asia Afrika No.22', 'Bandung', '+62 22 0000 2222')
ON CONFLICT DO NOTHING;

-- Static FAQ
CREATE TABLE IF NOT EXISTS faqs (
  id SERIAL PRIMARY KEY,
  question TEXT NOT NULL,
  answer TEXT NOT NULL
);

INSERT INTO faqs (question, answer) VALUES
('Bagaimana cara mengembalikan barang?', 'Silakan hubungi customer support dalam 7 hari setelah barang diterima.')
ON CONFLICT DO NOTHING;
INSERT INTO faqs (question, answer) VALUES
('Metode pembayaran apa yang tersedia?', 'Kami menerima transfer bank, e-wallet, dan kartu kredit.')
ON CONFLICT DO NOTHING;
