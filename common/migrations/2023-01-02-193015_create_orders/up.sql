CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  squarespace_id VARCHAR UNIQUE NOT NULL,
  order_number VARCHAR NOT NULL,
  created_on VARCHAR NOT NULL,
  modified_on VARCHAR NOT NULL,
  channel VARCHAR NOT NULL,
  testmode BOOLEAN NOT NULL,
  customer_email VARCHAR NOT NULL,
  -- Just holding nested structures as JSON strings for now
  -- depending on limitations / features needed we may want to change this later
  billing_address JSONB NOT NULL,
  -- JSON encoded string
  shipping_address JSONB,
  fulfillment_status VARCHAR NOT NULL,
  -- JSON encoded string (not null, but can be an empty array)
  line_items JSONB[] NOT NULL,
  -- Array of JSONB structs
  internal_notes JSONB[] NOT NULL,
  -- Array of JSONB structs
  shipping_lines JSONB[] NOT NULL,
  -- JSON encoded string (not null, but can be an empty array)
  discount_lines JSONB[] NOT NULL,
  -- Array of JSONB structs (can be null)
  form_submission JSONB[],
  -- JSON encoded string (not null, but can be an empty array)
  fulfillments JSONB[] NOT NULL,

  -- MoneyValue
  -- All JSON encoded strings
  subtotal JSONB NOT NULL,
  shipping_total JSONB NOT NULL,
  discount_total JSONB NOT NULL,
  tax_total JSONB NOT NULL,
  refunded_total JSONB NOT NULL,
  grand_total JSONB NOT NULL,

  -- Regular fields
  channel_name VARCHAR NOT NULL,
  external_order_reference VARCHAR,
  fulfilled_on VARCHAR,
  price_tax_interpretation VARCHAR
)
