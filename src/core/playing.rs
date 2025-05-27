use crate::core::{
    types::GameContract,
    types::{Card, CardSuit},
};

use super::{
    actions::{GameAction, GameActionKind},
    choosing::PlayerResponseState,
    game::{Game, GameError, GameState, PlayerScore, get_third, turn_inc},
    types::GameContractData,
};

#[derive(Debug)]
pub struct PlayingState {
    contract: GameContractData,
    declarer: usize,
    player_responses: [PlayerResponseState; 3],
    round_wins: [u32; 3],
    round_state: RoundState,
}

impl PlayingState {
    pub fn new(
        contract: GameContractData,
        declarer: usize,
        player_responses: [PlayerResponseState; 3],
    ) -> Self {
        Self {
            contract,
            declarer,
            player_responses,
            round_wins: Default::default(),
            round_state: RoundState::default(),
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

    fn rounds(&self) -> u32 {
        self.round_wins.iter().sum()
    }
}

#[derive(Debug, Default)]
pub struct RoundState {
    played_cards: [Option<Card>; 3],
    suit: Option<CardSuit>,
}

impl RoundState {
    fn is_round_over(&self) -> bool {
        for card in self.played_cards {
            if card.is_none() {
                return false;
            }
        }

        true
    }

    fn round_winner(self, trump: Option<CardSuit>) -> usize {
        if let Some(trump_suit) = trump {
            let trump_winner = self.winner_in_suit(trump_suit);

            if let Some(trump_winner) = trump_winner {
                return trump_winner;
            }
        }

        let winner = self.winner_in_suit(self.suit.unwrap());
        winner.unwrap()
    }

    fn winner_in_suit(&self, suit: CardSuit) -> Option<usize> {
        self.played_cards
            .iter()
            .map(|&x| {
                if x.is_some() && x.unwrap().suit == suit {
                    Some(x.unwrap().value)
                } else {
                    None
                }
            })
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
    }

    fn play_card(mut self, card: Card, player: usize) -> Self {
        debug_assert_eq!(
            self.played_cards[player], None,
            "Should not be able to play card twice"
        );

        self.played_cards[player] = Some(card);
        if self.suit.is_none() {
            self.suit = Some(card.suit)
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
        self.state.round_state = self.state.round_state.play_card(card, self.turn);

        if self.state.round_state.is_round_over() {
            self = self.register_win_and_reset_round();
        }

        self.to_next()
    }

    fn to_next(mut self) -> GameState {
        if self.is_hand_over() {
            let first = self.first;
            let score = self.calculate_new_scores();
            GameState::Bidding(Game::new_starting_state(turn_inc(first), score))
        } else {
            self.turn = self.next_turn();
            GameState::Playing(self)
        }
    }

    fn next_turn(&self) -> usize {
        let mut turn = turn_inc(self.turn);
        for _ in 0..3 {
            if self.state.player_responses[turn] != PlayerResponseState::Rejected {
                return turn;
            }

            turn = turn_inc(turn);
        }

        panic!("There should be at least one more person who has not passed bid.");
    }

    fn calculate_new_scores(mut self) -> [PlayerScore; 3] {
        self.score[self.state.declarer] =
            self.updated_declarer_score(self.score[self.state.declarer]);

        let goer1 = turn_inc(self.state.declarer);
        let goer2 = turn_inc(goer1);

        self.score[goer1] = self.updated_goer_score(goer1, self.score[goer1]);
        self.score[goer2] = self.updated_goer_score(goer2, self.score[goer2]);

        self.score
    }

    fn updated_declarer_score(&self, declarer_score: PlayerScore) -> PlayerScore {
        if self.declarer_passed() {
            declarer_score.apply_pass(self.state.contract)
        } else {
            declarer_score.apply_fail(self.state.contract)
        }
    }

    fn declarer_passed(&self) -> bool {
        self.state.round_wins[self.state.declarer] >= 6
    }

    fn updated_goer_score(&self, goer: usize, goer_score: PlayerScore) -> PlayerScore {
        if self.state.player_responses[goer] == PlayerResponseState::Called {
            return goer_score;
        }

        if self.goer_passed(goer) {
            let goer_score = goer_score.apply_soups(
                self.state.contract,
                self.state.round_wins[goer],
                self.soups(goer),
            );

            if self.state.player_responses[goer] == PlayerResponseState::Caller {
                self.caller_goer_score(goer, goer_score)
            } else {
                goer_score
            }
        } else {
            goer_score.apply_fail(self.state.contract)
        }
    }

    fn caller_goer_score(&self, goer: usize, goer_score: PlayerScore) -> PlayerScore {
        let goer2 = get_third(goer, self.state.declarer);
        goer_score.apply_soups(
            self.state.contract,
            self.state.round_wins[goer2],
            self.soups(goer),
        )
    }

    fn soups(&self, goer: usize) -> usize {
        let goer2 = get_third(goer, self.state.declarer);

        if self.state.declarer > goer2 { 1 } else { 0 }
    }

    fn goer_passed(&self, goer: usize) -> bool {
        let goer2 = get_third(self.state.declarer, goer);

        self.state.round_wins[goer] >= 2
            || self.state.round_wins[goer] + self.state.round_wins[goer2] >= 4
    }

    fn is_hand_over(&self) -> bool {
        self.state.rounds() == 10
    }

    fn register_win_and_reset_round(mut self) -> Self {
        let trump = self.state.trump();
        let round_winner = self.state.round_state.round_winner(trump);
        self.state.round_wins[round_winner] += 1;
        self.state.round_state = RoundState::default();

        self
    }
}
