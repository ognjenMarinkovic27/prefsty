use common::create_state_and_execute_actions;
use prefsty::core::{
    actions::{GameAction, GameActionKind},
    bidding::bidding::BiddingState,
    game::{Game, GameState},
    types::GameContract,
};

mod common;

#[test]
fn validate_bid() {
    let state = GameState::Bidding(<Game<BiddingState>>::new_starting_state(
        0,
        Default::default(),
    ));
    let bid_action = GameAction::new(0, GameActionKind::Bid);

    let is_valid = state.validate(&bid_action);
    assert_eq!(is_valid, true)
}

#[test]
fn validate_bid_wrong_turn() {
    let state = GameState::Bidding(<Game<BiddingState>>::new_starting_state(
        1,
        Default::default(),
    ));
    let bid_action = GameAction::new(0, GameActionKind::Bid);

    let is_valid = state.validate(&bid_action);
    assert_eq!(is_valid, false)
}

#[test]
fn validate_claim_nobid() {
    let state: GameState = GameState::Bidding(<Game<BiddingState>>::new_starting_state(
        0,
        Default::default(),
    ));

    let invalid_action = GameAction::new(0, GameActionKind::ClaimNoBid);
    assert_eq!(state.validate(&invalid_action), true);
}

#[test]
fn validate_invalid_action_type() {
    let state = GameState::Bidding(<Game<BiddingState>>::new_starting_state(
        0,
        Default::default(),
    ));

    let invalid_action = GameAction::new(0, GameActionKind::DeclareContre);
    assert_eq!(state.validate(&invalid_action), false);
}

#[test]
fn bid_pass_pass() {
    let bid_actions = [
        GameAction::new(0, GameActionKind::Bid),
        GameAction::new(1, GameActionKind::PassBid),
        GameAction::new(2, GameActionKind::PassBid),
    ];

    let state = create_state_and_execute_actions(0, bid_actions.into());

    match state {
        GameState::ChoosingCards(game) => assert_eq!(game.contract_bid(), GameContract::Spades),
        _ => panic!("State should be choosing cards"),
    }
}

#[test]
fn pass_pass_bid() {
    let bid_actions = [
        GameAction::new(0, GameActionKind::PassBid),
        GameAction::new(1, GameActionKind::PassBid),
        GameAction::new(2, GameActionKind::Bid),
    ];

    let state = create_state_and_execute_actions(0, bid_actions.into());

    match state {
        GameState::ChoosingCards(game) => assert_eq!(game.contract_bid(), GameContract::Spades),
        _ => panic!("State should be choosing cards"),
    }
}

#[test]
fn pass_pass_pass() {
    let bid_actions = [
        GameAction::new(0, GameActionKind::PassBid),
        GameAction::new(1, GameActionKind::PassBid),
        GameAction::new(2, GameActionKind::PassBid),
    ];

    let state = create_state_and_execute_actions(0, bid_actions.into());

    match state {
        GameState::Bidding(game) => {
            assert_eq!(game.first, 1);
            assert_eq!(game.turn, 1);
        }
        _ => panic!("State should be choosing cards"),
    }
}

// Test bid escalation sequence
#[test]
fn bid_escalation_sequence() {
    let bid_actions = [
        GameAction::new(0, GameActionKind::Bid), // Spades
        GameAction::new(1, GameActionKind::Bid), // Clubs
        GameAction::new(2, GameActionKind::Bid), // Hearts
        GameAction::new(0, GameActionKind::PassBid),
        GameAction::new(1, GameActionKind::PassBid),
    ];

    let state = create_state_and_execute_actions(0, bid_actions.into());

    match state {
        GameState::ChoosingCards(game) => {
            assert_eq!(game.contract_bid(), GameContract::Hearts);
            assert_eq!(game.turn, 2, "It should be the bid winner's turn")
        }
        _ => panic!("State should be choosing cards"),
    }
}

// Test bid stealing mechanism after full circle
#[test]
fn bid_stealing_after_full_circle() {
    let bid_actions = [
        GameAction::new(0, GameActionKind::Bid),
        GameAction::new(1, GameActionKind::Bid),
        GameAction::new(2, GameActionKind::Bid),
        GameAction::new(0, GameActionKind::PassBid),
        GameAction::new(1, GameActionKind::Bid),
        GameAction::new(2, GameActionKind::PassBid),
    ];

    let state = create_state_and_execute_actions(0, bid_actions.into());

    match state {
        GameState::ChoosingCards(game) => {
            assert_eq!(game.contract_bid(), GameContract::Hearts);
            assert_eq!(game.turn, 1, "It should be the bid winner's turn")
        }
        _ => panic!("State should be choosing cards"),
    }
}

// Test that players who passed can't bid again
#[test]
fn passed_players_cannot_bid() {
    let bid_actions = [
        GameAction::new(0, GameActionKind::Bid),
        GameAction::new(1, GameActionKind::PassBid),
        GameAction::new(2, GameActionKind::Bid),
        GameAction::new(0, GameActionKind::Bid),
    ];

    let state = create_state_and_execute_actions(0, bid_actions.into());

    // Player 1 has passed, so their turn should be skipped
    match &state {
        GameState::Bidding(game) => assert_eq!(game.turn, 2), // Should skip to player 2
        _ => panic!("State should still be bidding"),
    }
}

// Test alternating bid and pass sequence
#[test]
fn alternating_bid_pass_sequence() {
    let bid_actions = [
        GameAction::new(0, GameActionKind::Bid),
        GameAction::new(1, GameActionKind::PassBid),
        GameAction::new(2, GameActionKind::Bid),
        GameAction::new(0, GameActionKind::PassBid),
    ];

    let state = create_state_and_execute_actions(0, bid_actions.into());

    let pass_last_action = GameAction::new(2, GameActionKind::PassBid);
    let is_valid_to_pass_last = state.validate(&pass_last_action);

    assert_eq!(
        is_valid_to_pass_last, false,
        "The last bidder remaining should not be able to pass"
    )
}

#[test]
fn turn_should_move_after_bid() {
    let bid_actions = [GameAction::new(0, GameActionKind::Bid)];

    let state = create_state_and_execute_actions(0, bid_actions.into());

    // Turn should move to next player, not stay with bidder
    match &state {
        GameState::Bidding(game) => assert_ne!(game.turn, 0),
        _ => panic!("State should still be bidding"),
    }
}

// Test edge case: two players bid, third passes, first two continue
#[test]
fn two_active_bidders_one_passed() {
    let bid_actions = [
        GameAction::new(0, GameActionKind::Bid),
        GameAction::new(1, GameActionKind::Bid),
        GameAction::new(2, GameActionKind::PassBid),
        GameAction::new(0, GameActionKind::Bid),
        GameAction::new(1, GameActionKind::Bid),
        GameAction::new(0, GameActionKind::Bid),
        GameAction::new(1, GameActionKind::PassBid),
    ];

    let state = create_state_and_execute_actions(0, bid_actions.into());

    match state {
        GameState::ChoosingCards(game) => assert_eq!(game.contract_bid(), GameContract::Hearts),
        _ => panic!("State should be choosing cards"),
    }
}

#[test]
pub fn start_with_nobid() {
    let bid_actions = [GameAction::new(0, GameActionKind::ClaimNoBid)];
    let state = create_state_and_execute_actions(0, bid_actions.into());

    match state {
        GameState::NoBidPlayClaim(_) => {}
        _ => panic!("State should be no bid play claim"),
    }
}

#[test]
pub fn interrupt_bid_with_nobid() {
    let bid_actions = [
        GameAction::new(0, GameActionKind::Bid),
        GameAction::new(1, GameActionKind::ClaimNoBid),
    ];

    let state = create_state_and_execute_actions(0, bid_actions.into());

    let invalid_bid = GameAction::new(2, GameActionKind::Bid);
    let is_action_valid = state.validate(&invalid_bid);

    assert_eq!(
        is_action_valid, false,
        "Bidding after No Bid Claim should be invalid",
    );

    match state {
        GameState::NoBidPlayClaim(_) => {}
        _ => panic!("State should be no bid play claim"),
    }
}
