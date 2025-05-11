use crate::core::game::GameContract;

pub struct PlayingState {
    state: PlayingSubState,
    contract: GameContract,
    declarer_id: String,
}

pub enum PlayingSubState {
    RespondingToContract,
    
}