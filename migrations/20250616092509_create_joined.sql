CREATE TABLE IF NOT EXISTS joined (
    game_id UUID NOT NULL REFERENCES games(id),
    user_id UUID NOT NULL REFERENCES users(id),
    PRIMARY KEY (game_id, user_id)
)