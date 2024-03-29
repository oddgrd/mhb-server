CREATE TABLE IF NOT EXISTS boulders (
    id TEXT DEFAULT generate_ulid() NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    suggested_grade INT NOT NULL,
    published BOOL NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

