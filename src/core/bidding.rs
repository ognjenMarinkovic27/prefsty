pub mod bidding;
pub mod no_bid;
mod share;

use super::types::GameContract;

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
enum PlayerBidState {
    #[default]
    NoBid,
    Bid(GameContract),
    PassedBid,
    NoPlayClaim,
}

#[derive(Debug)]
pub struct Bid {
    value: GameContract,
    bidder_ind: usize,
}
