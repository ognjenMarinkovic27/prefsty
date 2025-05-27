use crate::core::game::turn_inc;

use super::PlayerBidState;

pub(super) fn next_turn(current_turn: usize, player_states: &[PlayerBidState]) -> usize {
    (1..=3)
        .map(|_| turn_inc(current_turn))
        .find(|&i| player_states[i] != PlayerBidState::PassedBid)
        .expect("At least one active player remains")
}

pub(super) fn count_passed(player_states: &[PlayerBidState]) -> usize {
    player_states
        .iter()
        .filter(|&&p| p == PlayerBidState::PassedBid)
        .count()
}

pub(super) fn no_bid_exists(player_states: &[PlayerBidState]) -> bool {
    player_states
        .iter()
        .find(|&&x| x == PlayerBidState::NoBid)
        .is_some()
}
