CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    public_id UUID NOT NULL,
    fullname VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    disabled_at TIMESTAMP,
    UNIQUE(public_id, email)
);

CREATE INDEX idx_users_public_id ON users(public_id);
