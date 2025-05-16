use prefsty::core::{
    actions::{GameAction, GameActionKind},
    game::Room,
};

fn main() {
    let room = Room::new();

    let res = room.game.validate(GameAction {
        player_ind: 0,
        kind: GameActionKind::Bid,
    });
    assert_eq!(res, true);
}
