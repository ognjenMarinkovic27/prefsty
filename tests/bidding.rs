use prefsty::core::{
    actions::{self, GameAction},
    game::Game,
};

#[test]
fn validate_bid() {
    let state = Game::new(0, Default::default(), Default::default());
    let action = GameAction {
        player_ind: 0,
        kind: actions::GameActionKind::Bid,
    };

    let is_valid = state.validate(&action);
    assert_eq!(is_valid, true)
}

#[test]
fn validate_bid_wrong_turn() {
    let state = Game::new(0, Default::default(), Default::default());
    let action = GameAction {
        player_ind: 1,
        kind: actions::GameActionKind::Bid,
    };

    let is_valid = state.validate(&action);
    assert_eq!(is_valid, false)
}

#[test]
fn max_bid_reached() {
    let state = Game::new(0, Default::default(), Default::default());
    let action = GameAction {
        player_ind: 0,
        kind: actions::GameActionKind::Bid,
    };

    let is_valid = state.validate(&action);

    todo!();

    assert_eq!(is_valid, true)
}
