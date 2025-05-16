use super::{
    game::{Game, GameState},
    types::{Card, GameContract},
};

pub struct RespondingToContractState {
    contract: GameContract,
    declarer_ind: usize,
    accepted: [bool; 3],
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
pub struct ChooseCardsAction {
    cards: [Card; 2],
}

impl ChooseCardsAction {
    pub fn validate(&self, player_ind: usize, game: &Game) -> bool {
        match &game.state {
            GameState::ChoosingCards(_) => self.is_contained(&game.hands[player_ind]),
            _ => false,
        }
    }

    fn is_contained(&self, hand: &[Card]) -> bool {
        for card in &self.cards {
            if !hand.contains(&card) {
                return false;
            }
        }

        true
    }
}

pub struct ChooseContractAction {
    contract: GameContract,
}

impl ChooseContractAction {
    pub fn validate(&self, game: &Game) -> bool {
        match &game.state {
            GameState::ChoosingContract(contract) => *contract <= self.contract,
            _ => false,
        }
    }
}

pub struct AcceptContractAction {}

impl AcceptContractAction {
    pub fn validate(&self, player_ind: usize, game: &Game) -> bool {
        match &game.state {
            GameState::RespondingToContract(responding_state) => {
                debug_assert!(
                    responding_state.declarer_ind != player_ind,
                    "Player should not be responding to his own contract"
                );

                true
            }
            _ => false,
        }
    }
}

pub struct RejectContractAction {}

impl RejectContractAction {
    pub fn validate(&self, player_ind: usize, game: &Game) -> bool {
        match &game.state {
            GameState::RespondingToContract(responding_state) => {
                debug_assert!(
                    responding_state.declarer_ind != player_ind,
                    "Player should not be responding to his own contract"
                );

                true
            }
            _ => false,
        }
    }
}

pub struct CallForHelpAction {}

impl CallForHelpAction {
    pub fn validate(&self, player_ind: usize, game: &Game) -> bool {
        match &game.state {
            GameState::HelpOrContreToContract(responding_state) => {
                CallForHelpAction::can_call(player_ind, responding_state)
            }
            _ => false,
        }
    }

    fn can_call(player_ind: usize, responding_state: &RespondingToContractState) -> bool {
        let teammate_ind = get_third_ind(player_ind, responding_state.declarer_ind);
        !responding_state.accepted[teammate_ind]
    }
}

pub struct DeclareContreAction {}

impl DeclareContreAction {
    pub fn validate(&self, player_ind: usize, game: &Game) -> bool {
        match &game.state {
            GameState::HelpOrContreToContract(responding_state) => {
                Self::validate_help_or_contre(responding_state, player_ind)
            }
            GameState::ContreDeclared(contre_responding_state) => {
                Self::validate_contre_declared(contre_responding_state, player_ind)
            }
            _ => false,
        }
    }

    fn validate_help_or_contre(
        responding_state: &RespondingToContractState,
        player_ind: usize,
    ) -> bool {
        DeclareContreAction::assert_ids_differ(responding_state.declarer_ind, player_ind);

        true
    }

    fn validate_contre_declared(
        contre_responding_state: &ContreDeclaredState,
        player_ind: usize,
    ) -> bool {
        let last_declarer_id = match &contre_responding_state.contre_level {
            ContreLevel::Contre | ContreLevel::Subcontre => {
                contre_responding_state.contract_declarer_ind
            }
            ContreLevel::Recontre | ContreLevel::FuckYouContre => {
                contre_responding_state.contre_declarer_ind
            }
        };

        DeclareContreAction::assert_ids_differ(last_declarer_id, player_ind);

        true
    }

    fn assert_ids_differ(last_declarer_ind: usize, player_ind: usize) {
        debug_assert!(
            last_declarer_ind != player_ind,
            "Player should not be able to respond to his own contre"
        );
    }
}

fn get_third_ind(ind1: usize, ind2: usize) -> usize {
    // Indexes can be 0, 1 and 2
    return 3 - ind1 - ind2;
}
