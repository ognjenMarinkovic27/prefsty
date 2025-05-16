#[derive(PartialEq)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
}

#[derive(PartialEq)]
pub enum CardSuit {
    Spades,
    Diamonds,
    Hearts,
    Clubs,
}

#[derive(PartialEq)]
pub enum CardValue {
    Seven,
    Eight,
    Nine,
    Ten,
    King,
    Queen,
    Ace,
}

#[derive(PartialEq, PartialOrd)]
pub enum GameContract {
    Spades = 2,
    Diamonds = 3,
    Hearts = 4,
    Clubs = 5,
    Betl = 6,
    Sans = 7,
}
