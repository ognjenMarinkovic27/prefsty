use super::{
    actions::{self, GameAction, GameActionKind},
    game::{Game, GameState},
    types::{Card, GameContract},
};

pub struct ChoosingCardsState;

impl Game<ChoosingCardsState> {
    pub fn validate(&self, action: GameAction) -> bool {
        match action.kind {
            GameActionKind::ChooseCards(cards) => self.is_contained(action.player_ind, &cards),
            _ => false,
        }
    }

    fn is_contained(&self, player_ind: usize, cards: &[Card]) -> bool {
        for card in cards {
            if !self.hands[player_ind].contains(&card) {
                return false;
            }
        }

        true
    }
}

pub struct ChoosingContractState {
    contract_bid: GameContract,
}

impl Game<ChoosingContractState> {
    pub fn validate(&self, action: GameAction) -> bool {
        match action.kind {
            GameActionKind::ChooseContract(contract) => contract > self.state.contract_bid,
            _ => false,
        }
    }
}

pub struct RespondingToContractState {
    contract: GameContract,
    declarer_ind: usize,
    accepted: [bool; 3],
}

impl Game<RespondingToContractState> {
    pub fn validate(&self, action: GameAction) -> bool {
        match action.kind {
            GameActionKind::AcceptContract | GameActionKind::RejectContract => {
                debug_assert!(
                    self.state.declarer_ind != action.player_ind,
                    "Player should not be responding to his own contract"
                );

                true
            }
            _ => false,
        }
    }
}

pub struct HelpOrContreToContractState {
    contract: GameContract,
    declarer_ind: usize,
    accepted: [bool; 3],
}

impl Game<HelpOrContreToContractState> {
    pub fn validate(&self, action: GameAction) -> bool {
        match action.kind {
            GameActionKind::CallForHelp => self.can_call(action.player_ind),
            GameActionKind::DeclareContre => self.can_declare_contre(action.player_ind),
            GameActionKind::PassHelpContre => true,
            _ => false,
        }
    }

    fn can_call(&self, player_ind: usize) -> bool {
        let teammate_ind = get_third_ind(player_ind, self.state.declarer_ind);
        !self.state.accepted[teammate_ind]
    }

    fn can_declare_contre(&self, player_ind: usize) -> bool {
        assert_ids_differ(self.state.declarer_ind, player_ind);

        true
    }
}

pub struct ContreDeclaredState {
    contract: GameContract,
    contract_declarer_ind: usize,
    contre_level: ContreLevel,
    contre_declarer_ind: usize,
}

pub enum ContreLevel {
    Contre,
    Recontre,
    Subcontre,
    FuckYouContre,
}

impl Game<ContreDeclaredState> {
    pub fn validate(&self, action: GameAction) -> bool {
        match action.kind {
            GameActionKind::DeclareContre => self.can_declare_contre(action.player_ind),
            GameActionKind::PassHelpContre => true,
            _ => false,
        }
    }

    fn can_declare_contre(&self, player_ind: usize) -> bool {
        let last_declarer_id = match &self.state.contre_level {
            ContreLevel::Contre | ContreLevel::Subcontre => self.state.contract_declarer_ind,
            ContreLevel::Recontre | ContreLevel::FuckYouContre => self.state.contre_declarer_ind,
        };

        assert_ids_differ(last_declarer_id, player_ind);

        true
    }
}

fn assert_ids_differ(last_declarer_ind: usize, player_ind: usize) {
    debug_assert!(
        last_declarer_ind != player_ind,
        "Player should not be able to respond to his own contre"
    );
}

fn get_third_ind(ind1: usize, ind2: usize) -> usize {
    // Indexes can be 0, 1 and 2
    return 3 - ind1 - ind2;
}
