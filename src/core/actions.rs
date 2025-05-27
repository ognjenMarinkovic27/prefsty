use super::types::{Card, GameContract};

pub struct GameAction {
    pub player: usize,
    pub kind: GameActionKind,
}

impl GameAction {
    pub fn new(player: usize, kind: GameActionKind) -> GameAction {
        GameAction { player, kind }
    }
}

pub enum GameActionKind {
    Bid,
    PassBid,
    ClaimNoBid,
    ChooseNoBidContract(GameContract),
    ChooseCards(CardChoice),
    ChooseContract(GameContract),
    AcceptContract,
    RejectContract,
    CallForHelp,
    DeclareContre,
    PassHelpContre,
    PlayCard(Card),
}

pub struct CardChoice {
    pub take_cards: Vec<Card>,
    pub discard_cards: Vec<Card>,
}
