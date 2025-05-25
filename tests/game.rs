#[cfg(test)]
pub fn create_game() {
    use prefsty::core::{bidding::BiddingState, game::Game};

    let game = <Game<BiddingState>>::new(0, Default::default());

    assert_eq!(game.first, 0);
    assert_eq!(game.turn, game.first);
}
