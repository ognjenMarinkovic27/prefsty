use common::create_state_and_execute_actions;
use prefsty::core::{
    actions::{GameAction, GameActionKind},
    game::GameState,
    types::GameContract,
};

mod common;

#[test]
fn no_bid_claims_lead_to_choice_state() {
    // First two players claim NoBid in turn
    let actions = vec![
        GameAction::new(0, GameActionKind::ClaimNoBid),
        GameAction::new(1, GameActionKind::ClaimNoBid),
        GameAction::new(2, GameActionKind::PassBid),
    ];
    let state = create_state_and_execute_actions(0, actions);

    match state {
        GameState::NoBidPlayChoice(game) => {
            // No contract has been chosen yet
            assert_eq!(game.contract_bid(), None);
            // And we know there is at least one more claim left, so we're still in Choice
        }
        other => panic!("Expected NoBidPlayChoice, got {:?}", other),
    }
}

#[test]
fn no_bid_choice_can_choose_contract() {
    // Drive into the choice state
    let actions = vec![
        GameAction::new(0, GameActionKind::ClaimNoBid),
        GameAction::new(1, GameActionKind::ClaimNoBid),
        GameAction::new(2, GameActionKind::PassBid),
        GameAction::new(
            0,
            GameActionKind::ChooseNoBidContract(GameContract::Diamonds),
        ),
    ];
    let state = create_state_and_execute_actions(0, actions);

    match state {
        GameState::NoBidPlayChoice(game) => {
            assert_eq!(game.contract_bid(), Some(GameContract::Diamonds));
        }
        other => panic!("Expected NoBidPlayChoice, got {:?}", other),
    }
}

#[test]
fn no_bid_choice_can_choose_contract_and_raise() {
    // Drive into the choice state
    let actions = vec![
        GameAction::new(0, GameActionKind::ClaimNoBid),
        GameAction::new(1, GameActionKind::ClaimNoBid),
        GameAction::new(2, GameActionKind::PassBid),
        GameAction::new(
            0,
            GameActionKind::ChooseNoBidContract(GameContract::Diamonds),
        ),
    ];

    let state = create_state_and_execute_actions(0, actions);

    let invalid_action =
        GameAction::new(1, GameActionKind::ChooseNoBidContract(GameContract::Spades));
    let is_valid = state.validate(&invalid_action);

    assert_eq!(is_valid, false);

    let raise_no_play_bid =
        GameAction::new(1, GameActionKind::ChooseNoBidContract(GameContract::Clubs));

    let state = state.apply(raise_no_play_bid).unwrap();

    match state {
        GameState::RespondingToContract(_) => {}
        other => panic!("Expected NoBidPlayChoice, got {:?}", other),
    }
}

#[test]
fn no_bid_choice_can_choose_contract_and_pass() {
    // Drive into the choice state
    let actions = vec![
        GameAction::new(0, GameActionKind::ClaimNoBid),
        GameAction::new(1, GameActionKind::ClaimNoBid),
        GameAction::new(2, GameActionKind::PassBid),
        GameAction::new(
            0,
            GameActionKind::ChooseNoBidContract(GameContract::Diamonds),
        ),
        GameAction::new(1, GameActionKind::PassBid),
    ];
    let state = create_state_and_execute_actions(0, actions);

    match state {
        GameState::RespondingToContract(_) => {}
        other => panic!("Expected NoBidPlayChoice, got {:?}", other),
    }
}
