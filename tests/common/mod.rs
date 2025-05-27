use prefsty::core::{
    actions::GameAction,
    bidding::bidding::BiddingState,
    game::{Game, GameState, Refas},
};

pub fn create_state_and_execute_actions(first_turn: usize, actions: Vec<GameAction>) -> GameState {
    let mut state = starting_state(first_turn, 60);
    for action in actions {
        state = state.apply(action).unwrap();
    }

    state
}

pub fn starting_state(first_turn: usize, starting_bulls: u32) -> GameState {
    GameState::Bidding(<Game<BiddingState>>::new_starting_state(
        first_turn,
        Default::default(),
        Refas::new((starting_bulls / 60) as usize),
    ))
}
