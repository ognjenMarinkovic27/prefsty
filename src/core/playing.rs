use crate::core::{
    types::GameContract,
    types::{Card, CardSuit},
};

use super::{
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

pub struct PlayCardAction {}

impl PlayCardAction {
    pub fn validate(&self, player_ind: usize, game: &Game) -> bool {
        match &game.state {
            GameState::Playing(playing_state) => {
                if Self::no_cards_played(playing_state) {
                    return true;
                }

                false
            }
            _ => false,
        }
    }

    fn no_cards_played(playing_state: &PlayingState) -> bool {
        let is_no_cards = playing_state.round_state.suit.is_none();

        if is_no_cards {
            for card in &playing_state.round_state.played_cards {
                debug_assert!(
                    card.is_none(),
                    "No cards should be played if round state suit is none"
                )
            }
        }

        is_no_cards
    }

    fn has_trump(player_ind: usize, hand: &[Card], trump: CardSuit) -> bool {
        let trump_card = hand.iter().find(|card| card.suit == trump);

        trump_card.is_some()
    }
}
