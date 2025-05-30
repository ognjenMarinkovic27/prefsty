use prefsty::core::{
    bidding::bidding::BiddingState,
    game::{Game, turn_dec, turn_inc},
};

#[test]
pub fn turn_incs() {
    let mut turn = 0;
    for i in 1..=3 {
        turn = turn_inc(turn);
        assert_eq!(turn, i % 3)
    }
}

#[test]
pub fn turn_decs() {
    let mut turn = 0;
    for i in 2..=0 {
        turn = turn_dec(turn);
        assert_eq!(turn, i % 3)
    }
}
