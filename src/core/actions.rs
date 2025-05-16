use super::types::{Card, GameContract};

pub struct GameAction {
    pub player_ind: usize,
    pub kind: GameActionKind,
}

pub enum GameActionKind {
    Bid,
    PassBid,
    ClaimNoBid,
    ChooseNoBidContract(GameContract),
    ChooseCards([Card; 2]),
    ChooseContract(GameContract),
    AcceptContract,
    RejectContract,
    CallForHelp,
    DeclareContre,
    PassHelpContre,
    PlayCard(Card),
}
