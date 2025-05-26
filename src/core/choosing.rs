use super::{
    actions::{CardChoice, GameAction, GameActionKind},
    game::{CardsInPlay, Game, GameError, GameState, PlayerScore},
    types::{Card, GameContract},
};

#[derive(Debug)]
pub struct ChoosingCardsState {
    contract_bid: GameContract,
}

impl Game<ChoosingCardsState> {
    pub fn new(
        first: usize,
        turn: usize,
        cards: CardsInPlay,
        score: [PlayerScore; 3],
        contract_bid: GameContract,
    ) -> Self {
        Self {
            first,
            turn,
            state: ChoosingCardsState { contract_bid },
            cards,
            score,
        }
    }

    pub fn validate(&self, action: &GameAction) -> bool {
        match &action.kind {
            GameActionKind::ChooseCards(choice) => self.validate_choose_cards(choice),
            _ => false,
        }
    }

    fn validate_choose_cards(&self, choice: &CardChoice) -> bool {
        if choice.take_cards.len() + choice.discard_cards.len() != 2 {
            return false;
        }

        if !self.hidden_cards_contain_take(&choice.take_cards) {
            return false;
        }

        if !self.hand_cards_contain_discard(&choice.discard_cards) {
            return false;
        }

        true
    }

    fn hidden_cards_contain_take(&self, take_cards: &[Card]) -> bool {
        Self::is_cards_contained(&self.cards.hidden, take_cards)
    }

    fn hand_cards_contain_discard(&self, discard_cards: &[Card]) -> bool {
        let current_player_ind = self.turn;
        Self::is_cards_contained(&self.cards.hands[current_player_ind], discard_cards)
    }

    fn is_cards_contained(container: &[Card], searched: &[Card]) -> bool {
        for card in searched {
            if !container.contains(card) {
                return false;
            }
        }

        true
    }

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        match action.kind {
            GameActionKind::ChooseCards(choice) => Ok(self.take_chosen_cards(choice)),
            _ => Err(GameError::InvalidAction),
        }
    }

    pub fn take_chosen_cards(mut self, choice: CardChoice) -> GameState {
        let current_player_hand = &mut self.cards.hands[self.turn];
        *current_player_hand = Self::remove_cards(current_player_hand, &choice.discard_cards);

        current_player_hand.extend_from_slice(&choice.take_cards);

        self.to_choose_contract()
    }

    fn remove_cards(container: &[Card], cards_to_remove: &[Card]) -> Vec<Card> {
        let mut vec_container = container.to_vec();
        for card in cards_to_remove {
            let card_pos = container.iter().position(|c| c == card).unwrap();
            vec_container.swap_remove(card_pos);
        }
        vec_container
    }

    fn to_choose_contract(self) -> GameState {
        GameState::ChoosingContract(<Game<ChoosingContractState>>::new(
            self.first,
            self.turn,
            self.cards,
            self.score,
            self.state.contract_bid,
        ))
    }

    pub fn contract_bid(&self) -> GameContract {
        self.state.contract_bid
    }
}

#[derive(Debug)]
pub struct ChoosingContractState {
    contract_bid: GameContract,
}

impl Game<ChoosingContractState> {
    fn new(
        first: usize,
        turn: usize,
        cards: CardsInPlay,
        score: [PlayerScore; 3],
        contract_bid: GameContract,
    ) -> Self {
        Self {
            state: ChoosingContractState { contract_bid },
            first,
            turn,
            cards,
            score,
        }
    }

    pub fn validate(&self, action: &GameAction) -> bool {
        match &action.kind {
            GameActionKind::ChooseContract(contract) => *contract > self.state.contract_bid,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct RespondingToContractState {
    contract: GameContract,
    declarer_ind: usize,
    accepted: [bool; 3],
}

impl Game<RespondingToContractState> {
    pub fn new(
        first: usize,
        turn: usize,
        cards: CardsInPlay,
        score: [PlayerScore; 3],
        contract: GameContract,
        declarer_ind: usize,
    ) -> Self {
        Game {
            state: RespondingToContractState {
                contract,
                declarer_ind,
                accepted: Default::default(),
            },
            first,
            turn,
            cards,
            score,
        }
    }

    pub fn validate(&self, action: &GameAction) -> bool {
        use GameActionKind::*;
        match &action.kind {
            AcceptContract | RejectContract => self.validate_response(action.player_ind),
            _ => false,
        }
    }

    fn validate_response(&self, player_ind: usize) -> bool {
        debug_assert!(
            self.state.declarer_ind != player_ind,
            "Player should not be responding to his own contract"
        );

        true
    }
}

#[derive(Debug)]
pub struct HelpOrContreToContractState {
    contract: GameContract,
    declarer_ind: usize,
    accepted: [bool; 3],
}

impl Game<HelpOrContreToContractState> {
    pub fn validate(&self, action: &GameAction) -> bool {
        match &action.kind {
            GameActionKind::CallForHelp => self.validate_call_for_help(action.player_ind),
            GameActionKind::DeclareContre => self.validate_declare_contre(action.player_ind),
            GameActionKind::PassHelpContre => self.validate_pass_help_contre(),
            _ => false,
        }
    }

    fn validate_call_for_help(&self, player_ind: usize) -> bool {
        let teammate_ind = get_third_ind(player_ind, self.state.declarer_ind);
        !self.state.accepted[teammate_ind]
    }

    fn validate_declare_contre(&self, player_ind: usize) -> bool {
        assert_ids_differ(self.state.declarer_ind, player_ind);

        true
    }

    fn validate_pass_help_contre(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct ContreDeclaredState {
    contract: GameContract,
    contract_declarer_ind: usize,
    contre_level: ContreLevel,
    contre_declarer_ind: usize,
}

#[derive(Debug)]
pub enum ContreLevel {
    Contre,
    Recontre,
    Subcontre,
    FuckYouContre,
}

impl Game<ContreDeclaredState> {
    pub fn validate(&self, action: &GameAction) -> bool {
        match action.kind {
            GameActionKind::DeclareContre => self.validate_declare_contre(action.player_ind),
            GameActionKind::PassHelpContre => self.validate_pass_help_contre(),
            _ => false,
        }
    }

    fn validate_declare_contre(&self, player_ind: usize) -> bool {
        let last_declarer_id = match &self.state.contre_level {
            ContreLevel::Contre | ContreLevel::Subcontre => self.state.contract_declarer_ind,
            ContreLevel::Recontre | ContreLevel::FuckYouContre => self.state.contre_declarer_ind,
        };

        assert_ids_differ(last_declarer_id, player_ind);

        true
    }

    fn validate_pass_help_contre(&self) -> bool {
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
