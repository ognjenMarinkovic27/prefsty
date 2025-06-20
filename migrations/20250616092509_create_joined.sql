CREATE TABLE joined (
  game_id UUID     NOT NULL REFERENCES games(id),
  idx     SMALLINT NOT NULL,
  user_id UUID     NULL     REFERENCES users(id),
  PRIMARY KEY (game_id, idx),
  UNIQUE     (game_id, user_id)
);
