use crate::core::{
    types::GameContract,
    types::{Card, CardSuit},
};

use super::{
    actions::{GameAction, GameActionKind},
    game::Game,
};

#[derive(Debug)]
pub struct PlayingState {
    contract: GameContract,
    declarer_ind: usize,
    trump: Option<CardSuit>,
    rounds: usize,
    round_state: RoundState,
}

#[derive(Debug)]
pub struct RoundState {
    played_cards: [Option<Card>; 3],
    suit: Option<CardSuit>,
}

impl Game<PlayingState> {
    pub fn validate(&self, action: &GameAction) -> bool {
        match action.kind {
            GameActionKind::PlayCard(card) => self.validate_play_card(action.player_ind, card),
            _ => false,
        }
    }

    fn validate_play_card(&self, player_ind: usize, card: Card) -> bool {
        if self.no_cards_played() || self.is_round_suit(card) {
            return true;
        } else if self.has_trump(player_ind) {
            return self.is_trump(card);
        }

        true
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

    fn is_round_suit(&self, card: Card) -> bool {
        if let Some(suit) = self.state.round_state.suit.as_ref() {
            *suit == card.suit
        } else {
            false
        }
    }

    fn has_trump(&self, player_ind: usize) -> bool {
        if let Some(trump_suit) = self.state.trump.as_ref() {
            let trump_card = self.cards.hands[player_ind]
                .iter()
                .find(|card| card.suit == *trump_suit);

            trump_card.is_some()
        } else {
            false
        }
    }

    fn is_trump(&self, card: Card) -> bool {
        if let Some(trump_suit) = self.state.trump.as_ref() {
            *trump_suit == card.suit
        } else {
            false
        }
    }
}
