use prefsty::core::{
    actions::{self, GameAction},
    bidding::BiddingState,
    game::{Game, GameState},
    types::GameContract,
};

#[test]
fn validate_bid() {
    let state = GameState::Bidding(<Game<BiddingState>>::new(0, Default::default()));
    let bid_action = GameAction::new(0, actions::GameActionKind::Bid);

    let is_valid = state.validate(&bid_action);
    assert_eq!(is_valid, true)
}

#[test]
fn validate_bid_wrong_turn() {
    let state = GameState::Bidding(<Game<BiddingState>>::new(0, Default::default()));
    let bid_action = GameAction::new(1, actions::GameActionKind::Bid);

    let is_valid = state.validate(&bid_action);
    assert_eq!(is_valid, false)
}

#[test]
fn bid_pass_pass() {
    let mut state = GameState::Bidding(<Game<BiddingState>>::new(0, Default::default()));
    let bid_actions = [
        GameAction::new(0, actions::GameActionKind::Bid),
        GameAction::new(1, actions::GameActionKind::PassBid),
        GameAction::new(2, actions::GameActionKind::PassBid),
    ];

    for action in bid_actions {
        state = state.apply(action).unwrap();
    }

    match state {
        GameState::ChoosingCards(game) => assert_eq!(game.contract_bid(), GameContract::Spades),
        _ => panic!("State should be choosing cards"),
    }
}

#[test]
fn pass_pass_bid() {
    let mut state = GameState::Bidding(<Game<BiddingState>>::new(0, Default::default()));
    let bid_actions = [
        GameAction::new(0, actions::GameActionKind::PassBid),
        GameAction::new(1, actions::GameActionKind::PassBid),
        GameAction::new(2, actions::GameActionKind::Bid),
    ];

    for action in bid_actions {
        state = state.apply(action).unwrap();
    }

    match state {
        GameState::ChoosingCards(game) => assert_eq!(game.contract_bid(), GameContract::Spades),
        _ => panic!("State should be choosing cards"),
    }
}
