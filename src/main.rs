use prefsty::core::{
    actions::{GameAction, GameActionKind},
    game::Room,
};

fn main() {
    let room = Room::new();

    let action = GameAction {
        player: 0,
        kind: GameActionKind::Bid,
    };
    let res = room.game.validate(&action);
    assert_eq!(res, true);
}
