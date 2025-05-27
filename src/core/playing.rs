use crate::core::{
    types::GameContract,
    types::{Card, CardSuit},
};

use super::{
    actions::{GameAction, GameActionKind},
    choosing::{ContreLevel, PlayerResponseState},
    game::{Game, GameError, GameState, get_third, turn_inc},
    types::GameContractData,
};

#[derive(Debug)]
pub struct PlayingState {
    contract: GameContractData,
    contre_level: ContreLevel,
    declarer: usize,
    responses: [PlayerResponseState; 3],
    tricks: [u32; 3],
    round: RoundState,
}

impl PlayingState {
    pub fn new(
        contract: GameContractData,
        contre_level: ContreLevel,
        declarer: usize,
        player_responses: [PlayerResponseState; 3],
    ) -> Self {
        Self {
            contract,
            contre_level,
            declarer,
            responses: player_responses,
            tricks: Default::default(),
            round: RoundState::default(),
        }
    }

    fn trump(&self) -> Option<CardSuit> {
        match self.contract.value {
            GameContract::Spades => Some(CardSuit::Spades),
            GameContract::Diamonds => Some(CardSuit::Diamonds),
            GameContract::Hearts => Some(CardSuit::Hearts),
            GameContract::Clubs => Some(CardSuit::Clubs),
            GameContract::Betl => None,
            GameContract::Sans => None,
        }
    }

    fn declarer_tricks(&self) -> u32 {
        self.tricks[self.declarer]
    }

    fn responder_tricks(&self) -> u32 {
        let responder1 = turn_inc(self.declarer);
        let responder2 = turn_inc(responder1);

        self.tricks[responder1] + self.tricks[responder2]
    }

    fn total_tricks(&self) -> u32 {
        self.tricks.iter().sum()
    }
}

#[derive(Debug, Default)]
pub struct RoundState {
    played: [Option<Card>; 3],
    lead_suit: Option<CardSuit>,
}

impl RoundState {
    fn is_round_over(&self) -> bool {
        self.played.iter().all(Option::is_some)
    }

    fn winner(&self, trump: Option<CardSuit>) -> usize {
        if let Some(trump_suit) = trump {
            if let Some(trump_winner) = self.highest_in_suit(trump_suit) {
                return trump_winner;
            }
        }

        let suit = self.lead_suit.expect("Lead suit always set");
        self.highest_in_suit(suit)
            .expect("At least one card in lead suit")
    }

    fn highest_in_suit(&self, suit: CardSuit) -> Option<usize> {
        self.played
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| c.filter(|c| c.suit == suit).map(|c| (i, c.value)))
            .max_by_key(|&(_, value)| value)
            .map(|(i, _)| i)
    }

    fn play_card(mut self, card: Card, player: usize) -> Self {
        debug_assert_eq!(
            self.played[player], None,
            "Should not be able to play card twice"
        );

        self.played[player] = Some(card);
        if self.lead_suit.is_none() {
            self.lead_suit = Some(card.suit)
        }

        self
    }
}

impl Game<PlayingState> {
    pub fn validate(&self, action: &GameAction) -> bool {
        match action.kind {
            GameActionKind::PlayCard(card) => self.validate_play_card(action.player, card),
            _ => false,
        }
    }

    fn validate_play_card(&self, player: usize, card: Card) -> bool {
        if !self.player_has_card(player, card) {
            return false;
        }

        if self.no_cards_played() || self.is_round_suit(card) {
            return true;
        } else if self.has_trump(player) {
            return self.is_trump(card);
        }

        true
    }

    fn player_has_card(&self, player: usize, card: Card) -> bool {
        self.cards.hands[player].contains(&card)
    }

    fn no_cards_played(&self) -> bool {
        let is_no_cards = self.state.round.lead_suit.is_none();

        if is_no_cards {
            for card in &self.state.round.played {
                debug_assert!(
                    card.is_none(),
                    "No cards should be played if round state suit is none"
                )
            }
        }

        is_no_cards
    }

    fn is_round_suit(&self, card: Card) -> bool {
        if let Some(suit) = self.state.round.lead_suit.as_ref() {
            *suit == card.suit
        } else {
            false
        }
    }

    fn has_trump(&self, player: usize) -> bool {
        if let Some(trump_suit) = self.state.trump().as_ref() {
            let trump_card = self.cards.hands[player]
                .iter()
                .find(|card| card.suit == *trump_suit);

            trump_card.is_some()
        } else {
            false
        }
    }

    fn is_trump(&self, card: Card) -> bool {
        if let Some(trump_suit) = self.state.trump().as_ref() {
            *trump_suit == card.suit
        } else {
            false
        }
    }

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        match action.kind {
            GameActionKind::PlayCard(card) => Ok(self.play_card(card)),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn play_card(mut self, card: Card) -> GameState {
        self.state.round = self.state.round.play_card(card, self.turn);
        self.remove_card_from_hand(card);

        self.to_next()
    }

    fn remove_card_from_hand(&mut self, card: Card) {
        let hand = &mut self.cards.hands[self.turn];
        if let Some(pos) = hand.iter().position(|&c| c == card) {
            hand.swap_remove(pos);
        }
    }

    fn to_next(mut self) -> GameState {
        if self.state.round.is_round_over() {
            self.end_round();
            self.to_next_after_round()
        } else {
            self.turn = self.next_turn();
            GameState::Playing(self)
        }
    }

    fn to_next_after_round(mut self) -> GameState {
        if self.is_hand_over() {
            let first = self.first;
            self.compute_scores();
            GameState::Bidding(Game::new_starting_state(turn_inc(first), self.score))
        } else {
            GameState::Playing(self)
        }
    }

    fn is_hand_over(&self) -> bool {
        match self.state.contract.value {
            GameContract::Betl => self.state.declarer_tricks() > 0,
            _ => self.state.total_tricks() == 10 || self.state.responder_tricks() >= 5,
        }
    }

    fn next_turn(&self) -> usize {
        (1..=3)
            .map(|_| turn_inc(self.turn))
            .find(|&i| self.state.responses[i] != PlayerResponseState::Rejected)
            .expect("At least one active player remains")
    }

    fn compute_scores(&mut self) {
        self.update_declarer_score();
        let responders = [
            turn_inc(self.state.declarer),
            turn_inc(turn_inc(self.state.declarer)),
        ];

        if self.state.contract.value == GameContract::Betl && self.state.declarer_tricks() > 0 {
            /*
               Comment of Laziness:
               Force tricks to 5 so betl fail soups is calculated properly
            */
            for responder in responders {
                self.state.tricks[responder] = 5;
            }
        }

        // Two "responders"
        for responder in responders {
            self.update_responder_score(responder);
        }
    }

    fn update_declarer_score(&mut self) {
        let declarer_score = &mut self.score[self.state.declarer];

        let pass_condition = match self.state.contract.value {
            GameContract::Betl => self.state.tricks[self.state.declarer] == 0,
            _ => self.state.tricks[self.state.declarer] >= 6,
        };

        declarer_score.apply_result(self.state.contract, pass_condition, self.state.contre_level)
    }

    fn update_responder_score(&mut self, responder: usize) {
        use PlayerResponseState::*;

        let declarer = self.state.declarer;
        let partner = get_third(declarer, responder);
        let responder_state = self.state.responses[responder];

        let responder_tricks = self.state.tricks[responder];
        let partner_tricks = self.state.tricks[partner];
        let total_tricks = responder_tricks + partner_tricks;

        let score = &mut self.score[responder];

        match responder_state {
            NoResponse => panic!("Responder should not be in No Response state"),
            Rejected | Called => {
                // These players don't score anything directly.
            }

            Caller => {
                // Caller takes credit for both players' tricks, must get 4 total
                let passed = total_tricks >= 4;
                let soups = if declarer > partner { 1 } else { 0 };

                score.apply_soups(
                    self.state.contract,
                    responder_tricks,
                    soups,
                    self.state.contre_level,
                );
                score.apply_soups(
                    self.state.contract,
                    partner_tricks,
                    soups,
                    self.state.contre_level,
                );

                if !passed {
                    score.apply_result(self.state.contract, false, self.state.contre_level);
                }
            }

            Contrer => {
                // Contrer must get 5 tricks in total (with the Called partner), and scores all rewards
                let passed = total_tricks >= 5;
                let soups = if declarer > partner { 1 } else { 0 };

                score.apply_soups(
                    self.state.contract,
                    responder_tricks,
                    soups,
                    self.state.contre_level,
                );
                score.apply_soups(
                    self.state.contract,
                    partner_tricks,
                    soups,
                    self.state.contre_level,
                );

                if !passed {
                    score.apply_result(self.state.contract, false, self.state.contre_level);
                }
            }

            Accepted => {
                // Normal case: pass if took 2 tricks OR combined 4
                let passed = responder_tricks >= 2 || total_tricks >= 4;
                let soups = if declarer > partner { 1 } else { 0 };

                score.apply_soups(
                    self.state.contract,
                    responder_tricks,
                    soups,
                    self.state.contre_level,
                );

                if !passed {
                    score.apply_result(self.state.contract, false, self.state.contre_level);
                }
            }
        }
    }

    fn end_round(&mut self) {
        let winner = self.state.round.winner(self.state.trump());
        self.state.tricks[winner] += 1;
        self.state.round = RoundState::default();
        self.turn = winner;
    }
}
