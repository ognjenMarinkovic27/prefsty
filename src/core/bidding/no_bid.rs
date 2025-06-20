use crate::core::actions::GameAction;
use crate::core::actions::GameActionKind;
use crate::core::choosing::RespondingToContractState;
use crate::core::game::Game;
use crate::core::game::GameError;
use crate::core::game::GameState;
use crate::core::types::GameContract;
use crate::core::types::GameContractData;
use crate::core::types::GameContractKind;

use super::Bid;
use super::PlayerBidState;
use super::share::next_turn;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NoBidClaimState {
    player_states: [PlayerBidState; 3],
}

impl NoBidClaimState {
    pub(super) fn new(player_states: [PlayerBidState; 3]) -> Self {
        Self { player_states }
    }
}

impl Game<NoBidClaimState> {
    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        self.validate(&action)?;

        debug_assert!(self.turn == action.player, "Should be validated beforehand");

        match action.kind {
            GameActionKind::ClaimNoBid => Ok(self.claim_no_bid()),
            GameActionKind::PassBid => Ok(self.pass_bid()),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn validate(&self, action: &GameAction) -> Result<(), GameError> {
        self.validate_turn(action)?;

        match action.kind {
            GameActionKind::ClaimNoBid => {
                if self.state.player_states[action.player] == PlayerBidState::NoBid {
                    Ok(())
                } else {
                    Err(GameError::BadAction)
                }
            }
            GameActionKind::PassBid => Ok(()),
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
        GameState::NoBidPlayChoice(self.into())
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

impl From<Game<NoBidClaimState>> for Game<NoBidChoiceState> {
    fn from(prev: Game<NoBidClaimState>) -> Self {
        let next_turn = prev.index_of_first_no_play_claimer();
        let claims = prev.number_of_no_play_claims();

        Self {
            state: NoBidChoiceState { bid: None, claims },
            first: prev.first,
            turn: next_turn,
            cards: prev.cards,
            score: prev.score,
            refas: prev.refas,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoBidChoiceState {
    bid: Option<Bid>,
    claims: usize,
}

impl NoBidChoiceState {
    pub fn new(bid: Option<Bid>, claims: usize) -> Self {
        Self { bid, claims }
    }
}

impl Game<NoBidChoiceState> {
    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        self.validate(&action)?;

        match action.kind {
            GameActionKind::ChooseNoBidContract(contract) => Ok(self.choose_no_bid(contract)),
            GameActionKind::PassBid => Ok(self.pass_bid()),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn validate(&self, action: &GameAction) -> Result<(), GameError> {
        self.validate_turn(action)?;

        match action.kind {
            GameActionKind::ChooseNoBidContract(contract) => self.validate_choose_no_bid(contract),
            GameActionKind::PassBid => self.validate_pass_bid(),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn validate_choose_no_bid(&self, contract: GameContract) -> Result<(), GameError> {
        if let Some(current_contract) = &self.state.bid {
            if contract > current_contract.value {
                Ok(())
            } else {
                Err(GameError::BadAction)
            }
        } else {
            Ok(())
        }
    }

    fn validate_pass_bid(&self) -> Result<(), GameError> {
        if self.state.bid.is_some() {
            Ok(())
        } else {
            Err(GameError::BadAction)
        }
    }

    fn choose_no_bid(mut self, contract: GameContract) -> GameState {
        self.state.bid = Some(Bid {
            value: contract,
            bidder: self.turn,
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
            GameState::RespondingToContract(self.into())
        }
    }

    pub fn contract_bid(&self) -> Option<GameContract> {
        self.state.bid.as_ref().map(|b| b.value)
    }
}

impl From<Game<NoBidChoiceState>> for Game<RespondingToContractState> {
    fn from(prev: Game<NoBidChoiceState>) -> Self {
        let Bid {
            value: contract,
            bidder: declarer,
        } = prev.state.bid.unwrap();

        Self {
            state: RespondingToContractState::new(
                GameContractData {
                    value: contract,
                    kind: GameContractKind::NoBid,
                },
                declarer,
            ),
            first: prev.first,
            turn: prev.turn,
            cards: prev.cards,
            score: prev.score,
            refas: prev.refas,
        }
    }
}
