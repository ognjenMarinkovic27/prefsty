use super::game::{turn_dec, turn_inc};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
}

#[derive(PartialEq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum CardSuit {
    Spades,
    Diamonds,
    Hearts,
    Clubs,
}

#[derive(PartialEq, Clone, Copy, Debug, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub enum CardValue {
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum GameContract {
    Spades = 2,
    Diamonds = 3,
    Hearts = 4,
    Clubs = 5,
    Betl = 6,
    Sans = 7,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum GameContractKind {
    Bid,
    NoBid,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct GameContractData {
    pub value: GameContract,
    pub kind: GameContractKind,
}

impl GameContract {
    pub fn first_to_play(&self, first: usize, declarer: usize) -> usize {
        match self {
            GameContract::Sans => turn_dec(declarer),
            _ => turn_inc(first),
        }
    }

    pub fn numerical_value(&self) -> u32 {
        match self {
            GameContract::Spades => 2,
            GameContract::Diamonds => 3,
            GameContract::Hearts => 4,
            GameContract::Clubs => 5,
            GameContract::Betl => 6,
            GameContract::Sans => 7,
        }
    }
}

impl GameContract {
    pub fn next(&self) -> Self {
        match self {
            GameContract::Spades => GameContract::Diamonds,
            GameContract::Diamonds => GameContract::Hearts,
            GameContract::Hearts => GameContract::Clubs,
            GameContract::Clubs => GameContract::Betl,
            GameContract::Betl => GameContract::Sans,
            GameContract::Sans => GameContract::Sans,
        }
    }

    pub fn is_last(&self) -> bool {
        *self == GameContract::Sans
    }
}
