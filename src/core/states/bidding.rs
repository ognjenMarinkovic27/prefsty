use std::collections::HashSet;
use crate::core::game::GameContract;

pub struct BiddingState {
    sub_state: BiddingSubState,
    already_bid: HashSet<String>
}

pub enum BiddingSubState {
    Regular(RegularBiddingState),
    NoBidPlayClaim(NoBidClaimState),
    NoBidPlayChoice(NoBidChoiceState),
}

pub struct RegularBiddingState {
    bid: u32,
    bidder_id: String,
}

pub struct NoBidClaimState {
    claimer_ids: Vec<String>,
}

pub struct NoBidChoiceState {
    contract: GameContract,
    declarer_id: String
}
