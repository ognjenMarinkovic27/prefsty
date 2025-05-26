use crate::core::game::turn_inc;

use super::PlayerBidState;

pub(super) fn next_turn(current_turn: usize, player_states: &[PlayerBidState]) -> usize {
    let mut turn = turn_inc(current_turn);
    for _ in 0..3 {
        if player_states[turn] != PlayerBidState::PassedBid {
            return turn;
        }

        turn = turn_inc(turn);
    }

    panic!("There should be at least one more person who has not passed bid.");
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
