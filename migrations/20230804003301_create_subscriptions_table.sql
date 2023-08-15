-- Add migration script here
-- We are keeping track of WHEN a subscription is created (subscribed_at)
-- We are enforcing email uniqueness at the database-level 
-- Enforcing that all fields should be populated
-- Using TEXT for email and name.
CREATE TABLE subscriptions (
  id uuid NOT NULL,
  PRIMARY KEY(id),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  subscribed_at timestamptz NOT NULL
);