CREATE TABLE games (
    id UUID PRIMARY KEY,
    state JSONB NOT NULL,
    created_by UUID NOT NULL
);
