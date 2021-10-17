CREATE TABLE IF NOT EXISTS option_data (
  id INT GENERATED ALWAYS AS IDENTITY,
  symbol TEXT NOT NULL,
  timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
  option_type TEXT NOT NULL CHECK (option_type in ('call', 'put')),
  strike DECIMAL NOT NULL,
  expiration_date TIMESTAMP WITH TIME ZONE NOT NULL,
  open_interest BIGINT,
  volume BIGINT,
  last DECIMAL,
  open DECIMAL,
  high DECIMAL,
  low DECIMAL,
  close DECIMAL,
  change DOUBLE PRECISION,
  underlying_price DECIMAL,
  implied_volatility DOUBLE PRECISION,
  PRIMARY KEY (id)
);
