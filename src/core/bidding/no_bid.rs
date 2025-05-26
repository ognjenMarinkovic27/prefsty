use crate::core::actions::GameAction;
use crate::core::actions::GameActionKind;
use crate::core::choosing::RespondingToContractState;
use crate::core::game::CardsInPlay;
use crate::core::game::Game;
use crate::core::game::GameError;
use crate::core::game::GameState;
use crate::core::game::PlayerScore;
use crate::core::game::turn_inc;
use crate::core::types::GameContract;

use super::Bid;
use super::PlayerBidState;
use super::share::count_passed;
use super::share::next_turn;
use super::share::no_bid_exists;

#[derive(Debug)]
pub struct NoBidClaimState {
    player_states: [PlayerBidState; 3],
}

impl Game<NoBidClaimState> {
    pub(super) fn new(
        first: usize,
        turn: usize,
        cards: CardsInPlay,
        score: [PlayerScore; 3],
        player_states: [PlayerBidState; 3],
    ) -> Self {
        Game {
            state: NoBidClaimState { player_states },
            first,
            turn,
            cards,
            score,
        }
    }

    pub fn validate(&self, action: &GameAction) -> bool {
        match action.kind {
            GameActionKind::ClaimNoBid => {
                self.state.player_states[action.player_ind] == PlayerBidState::NoBid
            }
            GameActionKind::PassBid => true,
            _ => false,
        }
    }

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        debug_assert!(
            self.turn == action.player_ind,
            "Should be validated beforehand",
        );

        match action.kind {
            GameActionKind::ClaimNoBid => Ok(self.claim_no_bid()),
            GameActionKind::PassBid => Ok(self.pass_bid()),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn claim_no_bid(mut self) -> GameState {
        self.state.player_states[self.turn] = PlayerBidState::NoPlayClaim;
        self.to_next()
    }

    fn pass_bid(mut self) -> GameState {
        self.state.player_states[self.turn] = PlayerBidState::PassedBid;
        self.to_next()
    }

    fn to_next(self) -> GameState {
        let next_turn = self.next_turn();
        if next_turn != self.first {
            self.to_next_no_bid_claim_state()
        } else {
            self.to_no_bid_choice_state()
        }
    }

    fn to_next_no_bid_claim_state(mut self) -> GameState {
        self.turn = self.next_turn();

        GameState::NoBidPlayClaim(self)
    }

    fn to_no_bid_choice_state(self) -> GameState {
        let next_turn = self.index_of_first_no_play_claimer();
        let claims = self.number_of_no_play_claims();

        GameState::NoBidPlayChoice(<Game<NoBidChoiceState>>::new(
            self.first, next_turn, self.cards, self.score, claims,
        ))
    }

    fn index_of_first_no_play_claimer(&self) -> usize {
        self.state
            .player_states
            .iter()
            .cycle()
            .skip(self.first)
            .position(|&x| x == PlayerBidState::NoPlayClaim)
            .unwrap()
    }

    fn number_of_no_play_claims(&self) -> usize {
        self.state
            .player_states
            .iter()
            .filter(|&&x| x == PlayerBidState::NoPlayClaim)
            .count()
    }

    fn next_turn(&self) -> usize {
        next_turn(self.turn, &self.state.player_states)
    }
}

#[derive(Debug)]
pub struct NoBidChoiceState {
    bid: Option<Bid>,
    claims: usize,
}

impl Game<NoBidChoiceState> {
    pub(super) fn new(
        first: usize,
        turn: usize,
        cards: CardsInPlay,
        score: [PlayerScore; 3],
        claims: usize,
    ) -> Self {
        Game {
            state: NoBidChoiceState { bid: None, claims },
            first,
            turn,
            cards,
            score,
        }
    }

    pub fn validate(&self, action: &GameAction) -> bool {
        match action.kind {
            GameActionKind::ChooseNoBidContract(contract) => {
                if let Some(current_contract) = &self.state.bid {
                    contract > current_contract.value
                } else {
                    true
                }
            }
            GameActionKind::PassBid => self.state.bid.is_some(),
            _ => false,
        }
    }

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        match action.kind {
            GameActionKind::ChooseNoBidContract(contract) => Ok(self.choose_no_bid(contract)),
            GameActionKind::PassBid => Ok(self.pass_bid()),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn choose_no_bid(mut self, contract: GameContract) -> GameState {
        self.state.bid = Some(Bid {
            value: contract,
            bidder_ind: self.turn,
        });

        self.to_next()
    }

    fn pass_bid(self) -> GameState {
        self.to_next()
    }

    fn to_next(mut self) -> GameState {
        self.state.claims -= 1;

        if self.state.claims > 0 {
            GameState::NoBidPlayChoice(self)
        } else {
            let Bid {
                value: contract,
                bidder_ind: declarer_ind,
            } = self.state.bid.unwrap();

            GameState::RespondingToContract(<Game<RespondingToContractState>>::new(
                self.first,
                turn_inc(self.turn),
                self.cards,
                self.score,
                contract,
                declarer_ind,
            ))
        }
    }

    pub fn contract_bid(&self) -> Option<GameContract> {
        self.state.bid.as_ref().map(|b| b.value)
    }
}
