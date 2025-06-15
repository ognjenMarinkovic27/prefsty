use prefsty::{
    core::{
        actions::{GameAction, GameActionKind},
        game::new_game,
    },
    persistence::{PgDB, model::GameModel},
};
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("Provide a DATABASE_URL to run prefsty");

    let pool = PgPool::connect(&db_url).await?;
    let db = PgDB::new(pool);

    let uuid = Uuid::new_v4();

    let game_model = GameModel {
        id: uuid,
        state: new_game(0, 120, 2),
    };

    db.create_game(game_model).await.unwrap();

    let game_model = db.load_game(uuid).await.unwrap();
    let game = game_model.state;
    game.apply(GameAction {
        player: 0,
        kind: GameActionKind::Bid,
    })
    .expect("should be able to bid");

    Ok(())
}
