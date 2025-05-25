use rand::{rng, seq::SliceRandom};

use super::{
    actions::GameAction,
    bidding::*,
    choosing::*,
    player::Player,
    playing::*,
    types::{Card, CardSuit, CardValue},
};

pub struct Room {
    pub game: GameState,
    players: Vec<Player>,
}

impl Room {
    pub fn new() -> Self {
        Room {
            game: GameState::Bidding(<Game<BiddingState>>::new(0, Default::default())),
            players: Vec::new(),
        }
    }
}

pub struct Game<S> {
    pub state: S,
    pub first: usize,
    pub turn: usize,
    pub cards: CardsInPlay,
    pub score: [PlayerScore; 3],
}

pub struct CardsInPlay {
    pub hands: [Vec<Card>; 3],
    pub hidden: [Card; 2],
}

impl CardsInPlay {
    /// Create and shuffle a full 32-card deck
    fn create_shuffled_deck() -> Vec<Card> {
        let mut deck = Self::create_deck();
        Self::shuffle_deck(&mut deck);
        deck
    }

    /// Generate a full 32-card deck: all suits and values
    fn create_deck() -> Vec<Card> {
        let suits = [
            CardSuit::Spades,
            CardSuit::Diamonds,
            CardSuit::Hearts,
            CardSuit::Clubs,
        ];
        let values = [
            CardValue::Seven,
            CardValue::Eight,
            CardValue::Nine,
            CardValue::Ten,
            CardValue::Jack,
            CardValue::Queen,
            CardValue::King,
            CardValue::Ace,
        ];

        let mut deck = Vec::with_capacity(suits.len() * values.len());
        for &suit in &suits {
            for &value in &values {
                deck.push(Card { suit, value });
            }
        }
        deck
    }

    /// Shuffle the given deck in place
    fn shuffle_deck(deck: &mut [Card]) {
        let mut rng = rng();
        deck.shuffle(&mut rng);
    }

    /// Deal n cards for each of `players` hands from the deck
    fn deal_hands(deck: &mut Vec<Card>, players: usize, cards_per_hand: usize) -> Vec<Vec<Card>> {
        (0..players)
            .map(|_| deck.drain(0..cards_per_hand).collect())
            .collect()
    }

    /// Draw `count` hidden cards from the deck
    fn deal_hidden(deck: &mut Vec<Card>, count: usize) -> Vec<Card> {
        deck.drain(0..count).collect()
    }

    /// Public API: deal random hands and hidden cards
    pub fn deal_random() -> Self {
        // 1) Create and shuffle deck
        let mut deck = Self::create_shuffled_deck();

        // 2) Deal to 3 players, 10 cards each
        let hands_vec = Self::deal_hands(&mut deck, 3, 10);

        // 3) Deal 2 hidden cards
        let hidden_vec = Self::deal_hidden(&mut deck, 2);

        // 4) Convert into fixed-size arrays
        let [hand1, hand2, hand3]: [Vec<Card>; 3] = hands_vec.try_into().unwrap();
        let hands = [hand1, hand2, hand3];
        let hidden = [hidden_vec[0], hidden_vec[1]];

        CardsInPlay { hands, hidden }
    }
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
    // Does this have to be so ugly?
    pub fn validate(&self, action: &GameAction) -> bool {
        match self {
            Self::Bidding(game) => game.validate_turn(action) && game.validate(action),
            Self::NoBidPlayClaim(game) => game.validate_turn(action) && game.validate(action),
            Self::NoBidPlayChoice(game) => game.validate_turn(action) && game.validate(action),
            Self::ChoosingCards(game) => game.validate_turn(action) && game.validate(action),
            Self::ChoosingContract(game) => game.validate_turn(action) && game.validate(action),
            Self::RespondingToContract(game) => game.validate_turn(action) && game.validate(action),
            Self::HelpOrContreToContract(game) => {
                game.validate_turn(action) && game.validate(action)
            }
            Self::ContreDeclared(game) => game.validate_turn(action) && game.validate(action),
            Self::Playing(game) => game.validate_turn(action) && game.validate(action),
        }
    }

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        match self {
            GameState::Bidding(game) => game.apply(action),
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

#[derive(Debug)]
pub enum GameError {
    InvalidAction,
}
