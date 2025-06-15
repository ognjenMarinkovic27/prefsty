use crate::core::{
    actions::{GameAction, GameActionKind},
    choosing::ChoosingCardsState,
    game::{CardsInPlay, Game, GameError, GameState, PlayerScore, Refas},
    types::GameContract,
};

use super::{
    Bid, PlayerBidState,
    no_bid::{NoBidChoiceState, NoBidClaimState},
    share::{count_passed, next_turn, no_bid_exists},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BiddingState {
    bid: Option<Bid>,
    can_steal_bid: bool,
    player_states: [PlayerBidState; 3],
}

impl Game<BiddingState> {
    pub fn new(first: usize, starting_score: u32, num_refas: usize) -> Self {
        <Game<BiddingState>>::new_starting_state(
            first,
            [
                PlayerScore::new(starting_score),
                PlayerScore::new(starting_score),
                PlayerScore::new(starting_score),
            ],
            Refas::new(num_refas),
        )
    }

    pub fn new_starting_state(first: usize, score: [PlayerScore; 3], refas: Refas) -> Self {
        Self {
            state: BiddingState {
                bid: None,
                can_steal_bid: false,
                player_states: Default::default(),
            },
            first,
            turn: first,
            cards: CardsInPlay::deal_random(),
            score,
            refas,
        }
    }

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        debug_assert!(self.turn == action.player, "Should be validated beforehand");

        self.validate(&action)?;

        match action.kind {
            GameActionKind::Bid => Ok(self.bid()),
            GameActionKind::PassBid => Ok(self.pass_bid()),
            GameActionKind::ClaimNoBid => Ok(self.claim_no_bid()),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn validate(&self, action: &GameAction) -> Result<(), GameError> {
        self.validate_turn(action)?;

        match action.kind {
            GameActionKind::Bid | GameActionKind::PassBid => self.validate_bid(action.player),
            GameActionKind::ClaimNoBid => self.validate_claim_nobid(action.player),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn validate_bid(&self, player: usize) -> Result<(), GameError> {
        if let Some(bid) = &self.state.bid {
            debug_assert!(
                bid.bidder != player,
                "Player should not be able to respond to his own bid"
            );
        }

        Ok(())
    }

    fn validate_claim_nobid(&self, player: usize) -> Result<(), GameError> {
        if self.state.player_states[player] == PlayerBidState::NoBid {
            Ok(())
        } else {
            Err(GameError::BadAction)
        }
    }

    fn bid(self) -> GameState {
        let registered_bid_state = self.register_bid();
        registered_bid_state.to_next()
    }

    fn next_bid_value(&self) -> GameContract {
        if let Some(current_bid) = &self.state.bid {
            self.next_after_existing_bid(current_bid.value)
        } else {
            GameContract::Spades
        }
    }

    fn register_bid(mut self) -> Self {
        let bidder = self.turn;
        let bid_value = self.next_bid_value();

        self.state.can_steal_bid = false;
        self.state.player_states[bidder] = PlayerBidState::Bid(bid_value);

        self.state.bid = Some(Bid {
            value: bid_value,
            bidder,
        });

        self
    }

    fn next_after_existing_bid(&self, current_value: GameContract) -> GameContract {
        if self.state.can_steal_bid {
            current_value
        } else {
            current_value.next()
        }
    }

    fn is_circle_completed(&self) -> bool {
        self.turn == self.first
    }

    fn pass_bid(mut self) -> GameState {
        self.state.player_states[self.turn] = PlayerBidState::PassedBid;

        self.to_next()
    }

    fn to_next(self) -> GameState {
        if self.no_bid_exists() {
            return self.to_next_bidding_state();
        }

        let passed_players = count_passed(&self.state.player_states);

        match passed_players {
            0 | 1 => self.to_next_bidding_state(),
            2 => self.to_choosing_cards(),
            _ => self.to_new_hand(),
        }
    }

    fn no_bid_exists(&self) -> bool {
        no_bid_exists(&self.state.player_states)
    }

    fn to_next_bidding_state(mut self) -> GameState {
        self.turn = self.next_turn();

        if self.is_circle_completed() {
            /*
               Comment of defeat:
               Whenever a circle completes, the next player to bid will not increase the bid. Instead,
               he gets to "steal" the current bid value.

               Example 1:
               2 -> 3 -> 4 -> Steal 4 -> 5

               Example 2:
               2 -> 3 -> 4 -> Pass -> Steal 4 -> 5
            */
            self.state.can_steal_bid = true;
        }

        GameState::Bidding(self)
    }

    fn to_choosing_cards(self) -> GameState {
        GameState::ChoosingCards(self.into())
    }

    fn to_new_hand(self) -> GameState {
        GameState::Bidding(<Game<BiddingState>>::new_starting_state(
            self.first + 1,
            self.score,
            self.refas,
        ))
    }

    fn claim_no_bid(mut self) -> GameState {
        self.state.player_states[self.turn] = PlayerBidState::NoPlayClaim;

        let next_turn = self.next_turn();
        if next_turn != self.first {
            GameState::NoBidPlayClaim(self.into())
        } else {
            GameState::NoBidPlayChoice(self.into())
        }
    }

    fn bids_as_passes(&self) -> [PlayerBidState; 3] {
        self.state.player_states.map(|x| {
            if let PlayerBidState::Bid(_) = x {
                PlayerBidState::PassedBid
            } else {
                x
            }
        })
    }

    fn next_turn(&self) -> usize {
        next_turn(self.turn, &self.state.player_states)
    }
}

impl From<Game<BiddingState>> for Game<ChoosingCardsState> {
    fn from(prev: Game<BiddingState>) -> Self {
        let bid = prev.state.bid.unwrap();

        Self {
            state: ChoosingCardsState::new(bid.value),
            first: prev.first,
            turn: bid.bidder,
            cards: prev.cards,
            score: prev.score,
            refas: prev.refas,
        }
    }
}

impl From<Game<BiddingState>> for Game<NoBidClaimState> {
    fn from(prev: Game<BiddingState>) -> Self {
        let next_turn = prev.next_turn();
        let bids_turned_to_passes = prev.bids_as_passes();

        Self {
            state: NoBidClaimState::new(bids_turned_to_passes),
            first: prev.first,
            turn: next_turn,
            cards: prev.cards,
            score: prev.score,
            refas: prev.refas,
        }
    }
}

impl From<Game<BiddingState>> for Game<NoBidChoiceState> {
    fn from(prev: Game<BiddingState>) -> Self {
        Self {
            state: NoBidChoiceState::new(None, 1),
            first: prev.first,
            turn: prev.turn,
            cards: prev.cards,
            score: prev.score,
            refas: prev.refas,
        }
    }
}
