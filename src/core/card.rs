pub struct Card {
    suit: CardSuit,
    value: CardValue,
}

pub enum CardSuit {
    Spades,
    Diamonds,
    Hearts,
    Clubs,
}

pub enum CardValue {
    Seven,
    Eight,
    Nine,
    Ten,
    King,
    Queen,
    Ace,
}