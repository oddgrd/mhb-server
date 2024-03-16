CREATE TABLE IF NOT EXISTS users (
    id TEXT DEFAULT generate_ulid() NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    avatar_url TEXT,
    google_id TEXT NOT NULL UNIQUE, -- TODO: make this a separate table so we can support multiple oauth providers.
    locale TEXT NOT NULL UNIQUE,
    access_token TEXT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
