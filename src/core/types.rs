#[derive(PartialEq, Clone)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
}

#[derive(PartialEq, Clone)]
pub enum CardSuit {
    Spades,
    Diamonds,
    Hearts,
    Clubs,
}

#[derive(PartialEq, Clone)]
pub enum CardValue {
    Seven,
    Eight,
    Nine,
    Ten,
    King,
    Queen,
    Ace,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum GameContract {
    Spades = 2,
    Diamonds = 3,
    Hearts = 4,
    Clubs = 5,
    Betl = 6,
    Sans = 7,
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

#[derive(Default, Clone)]
pub struct Hands([Hand; 3]);

#[derive(Default, Clone)]
pub struct Hand(Vec<Card>);
