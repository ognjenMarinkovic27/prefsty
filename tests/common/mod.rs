use prefsty::core::{
    actions::GameAction,
    bidding::bidding::BiddingState,
    game::{Game, GameState},
};

pub fn create_state_and_execute_actions(first_turn: usize, actions: Vec<GameAction>) -> GameState {
    let mut state = GameState::Bidding(<Game<BiddingState>>::new_starting_state(first_turn, Default::default()));
    for action in actions {
        state = state.apply(action).unwrap();
    }

    state
}
