use super::{
    actions::{GameAction, GameActionKind},
    game::Game,
    types::GameContract,
};

pub struct BiddingState {
    bid: GameContract,
    bidder_ind: usize,
    passed_bidding: [bool; 3],
    already_bid: [bool; 3],
}

impl Game<BiddingState> {
    pub fn validate(&self, action: GameAction) -> bool {
        match action.kind {
            GameActionKind::Bid | GameActionKind::PassBid => {
                debug_assert!(
                    self.state.bidder_ind != action.player_ind,
                    "Player should not be able to respond to his own bid"
                );

                debug_assert!(
                    self.state.bid < GameContract::Sans,
                    "If bid Sans is reached, state should have changed to ChoosingCards"
                );

                true
            }
            GameActionKind::ClaimNoBid => !self.state.already_bid[action.player_ind],
            _ => false,
        }
    }
}

pub struct NoBidClaimState {
    claimed: [bool; 3],
    already_bid: [bool; 3],
}

impl Game<NoBidClaimState> {
    pub fn validate(&self, action: GameAction) -> bool {
        match action.kind {
            GameActionKind::ClaimNoBid => !self.state.already_bid[action.player_ind],
            _ => false,
        }
    }
}

pub struct NoBidChoiceState {
    contract: GameContract,
    declarer_ind: usize,
}

impl Game<NoBidChoiceState> {
    pub fn validate(&self, action: GameAction) -> bool {
        match action.kind {
            GameActionKind::ChooseNoBidContract(contract) => contract > self.state.contract,
            _ => false,
        }
    }
}
