use super::{bidding::*, choosing::*, game::Game, playing::*};

pub struct GameAction {
    player_ind: usize,
    kind: GameActionKind,
}

impl GameAction {
    pub fn validate(&self, game: &Game) -> bool {
        if !self.is_player_turn(game.turn) {
            return false;
        }

        self.kind.validate(self.player_ind, game)
    }

    fn is_player_turn(&self, turn: usize) -> bool {
        return self.player_ind == turn;
    }
}

// TODO: use enum_dispatch crate
impl GameActionKind {
    fn validate(&self, player_ind: usize, game: &Game) -> bool {
        match &self {
            GameActionKind::Bid(bid_act) => bid_act.validate(player_ind, game),
            GameActionKind::PassBid(pass_bid_act) => pass_bid_act.validate(player_ind, game),
            GameActionKind::ClaimNoBid(claim_no_bid_act) => {
                claim_no_bid_act.validate(player_ind, game)
            }
            GameActionKind::ChooseNoBidContract(choose_no_bid_act) => {
                choose_no_bid_act.validate(game)
            }
            GameActionKind::ChooseCards(choose_cards_act) => {
                choose_cards_act.validate(player_ind, game)
            }
            GameActionKind::ChooseContract(choose_contract_act) => {
                choose_contract_act.validate(game)
            }
            GameActionKind::AcceptContract(accept_contract_act) => {
                accept_contract_act.validate(player_ind, game)
            }
            GameActionKind::RejectContract(reject_contract_act) => {
                reject_contract_act.validate(player_ind, game)
            }
            GameActionKind::CallForHelp(call_help_act) => call_help_act.validate(player_ind, game),
            GameActionKind::DeclareContre(declare_contre_act) => {
                declare_contre_act.validate(player_ind, game)
            }
            _ => false,
        }
    }
}

pub enum GameActionKind {
    Bid(BidAction),
    PassBid(PassBidAction),
    ClaimNoBid(ClaimNoBidAction),
    ChooseNoBidContract(ChooseNoBidAction),
    ChooseCards(ChooseCardsAction),
    ChooseContract(ChooseContractAction),
    AcceptContract(AcceptContractAction),
    RejectContract(RejectContractAction),
    CallForHelp(CallForHelpAction),
    DeclareContre(DeclareContreAction),
    PlayCard(PlayCardAction),
}
