-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    user_uuid TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT UNIQUE, -- Nullable
    phone_number TEXT UNIQUE, -- Nullable
    password TEXT NOT NULL,
    profile_picture_url TEXT,
    date_of_birth DATE,
    gender TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    user_type TEXT,
    is_email_verified BOOLEAN DEFAULT FALSE,
    is_phone_verified BOOLEAN DEFAULT FALSE,
    is_admin BOOLEAN DEFAULT FALSE,
    status TEXT DEFAULT 'active', -- Status can be 'active', 'inactive', or 'banned'
    CHECK (
        email IS NOT NULL OR phone_number IS NOT NULL
    )
);