use super::{
    actions::GameAction, bidding::*, choosing::*, player::Player, playing::*, types::Card,
};

pub struct Room {
    pub game: GameState,
    players: Vec<Player>,
}

impl Room {
    pub fn new() -> Self {
        Room {
            game: GameState::Bidding(Game::new(0, Default::default(), Default::default())),
            players: Vec::new(),
        }
    }
}

pub struct Game<S> {
    pub state: S,
    pub first: usize,
    pub turn: usize,
    pub hands: [Vec<Card>; 3],
    pub score: [PlayerScore; 3],
}

#[derive(Default)]
pub struct PlayerScore {
    bools: u32,
    soups: [u32; 2],
}

impl<StateType> Game<StateType> {
    pub fn validate_turn(&self, action: &GameAction) -> bool {
        self.is_player_turn(action.player_ind)
    }

    fn is_player_turn(&self, player_ind: usize) -> bool {
        return self.turn == player_ind;
    }

    pub fn turn_inc(&self) -> usize {
        (self.turn + 1) % 3
    }

    pub fn turn_dec(&self) -> usize {
        (self.turn + 2) % 3
    }
}

pub enum GameState {
    Bidding(Game<BiddingState>),
    NoBidPlayClaim(Game<NoBidClaimState>),
    NoBidPlayChoice(Game<NoBidChoiceState>),
    ChoosingCards(Game<ChoosingCardsState>),
    ChoosingContract(Game<ChoosingContractState>),
    RespondingToContract(Game<RespondingToContractState>),
    HelpOrContreToContract(Game<HelpOrContreToContractState>),
    ContreDeclared(Game<ContreDeclaredState>),
    Playing(Game<PlayingState>),
}

impl GameState {
    pub fn validate(&self, action: &GameAction) -> bool {
        match self {
            Self::Bidding(game) => game.validate(action),
            Self::NoBidPlayClaim(game) => game.validate(action),
            Self::NoBidPlayChoice(game) => game.validate(action),
            Self::ChoosingCards(game) => game.validate(action),
            Self::ChoosingContract(game) => game.validate(action),
            Self::RespondingToContract(game) => game.validate(action),
            Self::HelpOrContreToContract(game) => game.validate(action),
            Self::ContreDeclared(game) => game.validate(action),
            Self::Playing(game) => game.validate(action),
        }
    }

    pub fn apply(&self, action: &GameAction) -> GameState {
        match self {
            GameState::Bidding(game) => todo!(),
            GameState::NoBidPlayClaim(game) => todo!(),
            GameState::NoBidPlayChoice(game) => todo!(),
            GameState::ChoosingCards(game) => todo!(),
            GameState::ChoosingContract(game) => todo!(),
            GameState::RespondingToContract(game) => todo!(),
            GameState::HelpOrContreToContract(game) => todo!(),
            GameState::ContreDeclared(game) => todo!(),
            GameState::Playing(game) => todo!(),
        }
    }
}

pub enum GameError {
    InvalidAction,
}
