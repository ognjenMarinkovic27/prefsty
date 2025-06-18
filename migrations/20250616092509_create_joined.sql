CREATE TABLE joined (
  game_id UUID     NOT NULL,
  idx     SMALLINT NOT NULL,
  user_id UUID     NULL,
  PRIMARY KEY (game_id, idx),
  UNIQUE     (game_id, user_id)
);
