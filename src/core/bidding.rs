pub mod bidding;
pub mod no_bid;
mod share;

use super::types::GameContract;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy, Deserialize, Serialize)]
enum PlayerBidState {
    #[default]
    NoBid,
    Bid(GameContract),
    PassedBid,
    NoPlayClaim,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bid {
    value: GameContract,
    bidder: usize,
}
