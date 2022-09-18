CREATE TABLE IF NOT EXISTS media (
  id VARCHAR(21) PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  media_type VARCHAR(32) NOT NULL,
  catalogue VARCHAR(255) NOT NULL,
  release_date TIMESTAMP NOT NULL,
  purchase_date TIMESTAMP NOT NULL,
  media_condition VARCHAR(32) NOT NULL,
  sleeve_condition VARCHAR(32) NOT NULL,
  bought VARCHAR(32) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  modified_at TIMESTAMP NOT NULL,
  notes TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS media_artists_rel (
  id SERIAL PRIMARY KEY,
  media_id VARCHAR(21) NOT NULL,
  artist_id VARCHAR(21) NOT NULL
);

CREATE TABLE IF NOT EXISTS media_label_rel (
  id SERIAL PRIMARY KEY,
  media_id VARCHAR(21) NOT NULL,
  label_id VARCHAR(21) NOT NULL
);