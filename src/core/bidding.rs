use super::{
    game::{Game, GameState},
    types::GameContract,
};

pub struct BiddingState {
    bid: GameContract,
    bidder_ind: usize,
    passed_bidding: [bool; 3],
    already_bid: [bool; 3],
}

pub struct NoBidClaimState {
    claimed: [bool; 3],
    already_bid: [bool; 3],
}

pub struct NoBidChoiceState {
    contract: GameContract,
    declarer_ind: usize,
}

pub struct BidAction {}
pub struct PassBidAction {}

impl BidAction {
    pub fn validate(&self, player_ind: usize, game: &Game) -> bool {
        bid_validate(player_ind, game)
    }
}

impl PassBidAction {
    pub fn validate(&self, player_ind: usize, game: &Game) -> bool {
        bid_validate(player_ind, game)
    }
}

fn bid_validate(player_ind: usize, game: &Game) -> bool {
    match &game.state {
        GameState::Bidding(bidding) => {
            debug_assert!(
                bidding.bidder_ind != player_ind,
                "Player should not be able to respond to his own bid"
            );

            debug_assert!(
                bidding.bid < GameContract::Sans,
                "If bid Sans is reached, state should have changed to ChoosingCards"
            );

            true
        }
        GameState::Starting => true,
        _ => false,
    }
}

pub struct ClaimNoBidAction {}

impl ClaimNoBidAction {
    pub fn validate(&self, player_ind: usize, game: &Game) -> bool {
        match &game.state {
            GameState::Bidding(bidding) => !bidding.already_bid[player_ind],
            GameState::NoBidPlayClaim(no_bid_claim) => !no_bid_claim.already_bid[player_ind],
            GameState::Starting => true,
            _ => false,
        }
    }
}

pub struct ChooseNoBidAction {
    contract: GameContract,
}

impl ChooseNoBidAction {
    pub fn validate(&self, game: &Game) -> bool {
        match &game.state {
            GameState::NoBidPlayChoice(no_bid_choice) => self.contract > no_bid_choice.contract,
            _ => false,
        }
    }
}
