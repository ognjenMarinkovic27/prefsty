use crate::core::{
    types::GameContract,
    types::{Card, CardSuit},
};

use super::{
    actions::{GameAction, GameActionKind},
    game::{Game, GameState},
    player,
};

pub struct PlayingState {
    contract: GameContract,
    declarer_ind: usize,
    trump: Option<CardSuit>,
    rounds: usize,
    round_state: RoundState,
}

pub struct RoundState {
    played_cards: [Option<Card>; 3],
    suit: Option<CardSuit>,
}

impl Game<PlayingState> {
    pub fn validate(&self, action: GameAction) -> bool {
        match action.kind {
            GameActionKind::PlayCard(card) => false,
            _ => false,
        }
    }

    fn no_cards_played(&self) -> bool {
        let is_no_cards = self.state.round_state.suit.is_none();

        if is_no_cards {
            for card in &self.state.round_state.played_cards {
                debug_assert!(
                    card.is_none(),
                    "No cards should be played if round state suit is none"
                )
            }
        }

        is_no_cards
    }

    fn has_trump(hand: &[Card], trump: CardSuit) -> bool {
        let trump_card = hand.iter().find(|card| card.suit == trump);

        trump_card.is_some()
    }
}
