CREATE TABLE line_items (
  -- Using squarespace's id as the primary key
  id VARCHAR PRIMARY KEY,
  order_id VARCHAR REFERENCES orders(id) NOT NULL,
  variant_id VARCHAR,
  sku VARCHAR NOT NULL,
  weight REAL NOT NULL,
  width REAL NOT NULL,
  length REAL NOT NULL,
  height REAL NOT NULL,
  product_id VARCHAR,
  product_name VARCHAR,
  quantity INTEGER NOT NULL,
  unit_price_paid JSONB NOT NULL,
  variant_options JSONB[] NOT NULL,
  customizations JSONB[],
  image_url VARCHAR NOT NULL,
  line_item_type VARCHAR NOT NULL
)
