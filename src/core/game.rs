use rand::{rng, seq::SliceRandom};
use std::collections::VecDeque;

use super::{
    actions::GameAction,
    bidding::{
        bidding::BiddingState,
        no_bid::{NoBidChoiceState, NoBidClaimState},
    },
    choosing::*,
    playing::*,
    types::{Card, CardSuit, CardValue, GameContractData, GameContractKind},
};

use serde::{Deserialize, Serialize};

pub struct Room {
    pub game: GameState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game<S> {
    pub state: S,
    pub first: usize,
    pub turn: usize,
    pub cards: CardsInPlay,
    pub score: [PlayerScore; 3],
    pub refas: Refas,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Refas {
    active: VecDeque<Refa>,
    left: usize,
}

impl Refas {
    pub fn new(num_refas: usize) -> Self {
        Refas {
            active: VecDeque::default(),
            left: num_refas,
        }
    }

    pub fn has_refas_left(&self) -> bool {
        self.left > 0
    }

    pub fn add_active_refa(&mut self) {
        self.active.push_back(Refa::default());
    }

    pub fn has_active_refa(&self, player: usize) -> bool {
        self.active
            .iter()
            .find(|&x| x.used_by[player] == false)
            .is_some()
    }

    pub fn mark_active_refa(&mut self, player: usize) {
        let refa = self.active.iter_mut().find(|x| x.used_by[player] == false);

        if let Some(refa) = refa {
            refa.mark_used(player);

            if refa.is_done() {
                self.active.pop_front();
            }
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Refa {
    used_by: [bool; 3],
}

impl Refa {
    pub fn mark_used(&mut self, player: usize) {
        debug_assert_eq!(self.used_by[player], false, "can't be used twice");
        self.used_by[player] = true;
    }

    pub fn is_done(&self) -> bool {
        self.used_by.iter().all(|&x| x)
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct PlayerScore {
    bulls: u32,
    soups: [u32; 2],
}

impl PlayerScore {
    pub fn new(bulls: u32) -> Self {
        Self {
            bulls,
            soups: [0; 2],
        }
    }

    pub fn apply_result(
        &mut self,
        contract: GameContractData,
        is_passed: bool,
        contre: ContreLevel,
    ) {
        if is_passed {
            self.bulls -= Self::contract_value(contract, contre);
        } else {
            self.bulls += Self::contract_value(contract, contre);
        }
    }

    pub fn apply_soups(
        &mut self,
        contract: GameContractData,
        num_soups: u32,
        soups_ind: usize,
        contre: ContreLevel,
    ) {
        self.soups[soups_ind] += num_soups * Self::contract_value(contract, contre);
    }

    fn contract_value(contract: GameContractData, contre: ContreLevel) -> u32 {
        let contract_value = match contract.kind {
            GameContractKind::Bid => contract.value.numerical_value(),
            GameContractKind::NoBid => contract.value.numerical_value() + 2,
        };

        let contre_multipler = match contre {
            ContreLevel::NoContre => 1,
            ContreLevel::Contre => 2,
            ContreLevel::Recontre => 4,
            ContreLevel::Subcontre => 8,
            ContreLevel::FuckYouContre => 16,
        };

        contract_value * 2 * contre_multipler
    }
}

impl<StateType> Game<StateType> {
    pub fn validate_turn(&self, action: &GameAction) -> Result<(), GameError> {
        if self.is_player_turn(action.player) {
            Ok(())
        } else {
            Err(GameError::InvalidTurn)
        }
    }

    fn is_player_turn(&self, player: usize) -> bool {
        return self.turn == player;
    }
}

pub fn turn_inc(turn: usize) -> usize {
    (turn + 1) % 3
}

pub fn turn_dec(turn: usize) -> usize {
    (turn + 2) % 3
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        match self {
            GameState::Bidding(game) => game.apply(action),
            GameState::NoBidPlayClaim(game) => game.apply(action),
            GameState::NoBidPlayChoice(game) => game.apply(action),
            GameState::ChoosingCards(game) => game.apply(action),
            GameState::ChoosingContract(game) => game.apply(action),
            GameState::RespondingToContract(game) => game.apply(action),
            GameState::HelpOrContreToContract(game) => game.apply(action),
            GameState::ContreDeclared(game) => game.apply(action),
            GameState::Playing(game) => game.apply(action),
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    InvalidAction,
    BadAction,
    InvalidTurn,
}

pub fn get_third(ind1: usize, ind2: usize) -> usize {
    // Indexes can be 0, 1 and 2
    return 3 - ind1 - ind2;
}

pub fn new_game(first: usize, starting_score: u32, num_refas: usize) -> GameState {
    GameState::Bidding(<Game<BiddingState>>::new(first, starting_score, num_refas))
}
