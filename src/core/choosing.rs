use super::{
    actions::{CardChoice, GameAction, GameActionKind},
    game::{Game, GameError, GameState, get_third_ind, turn_inc},
    playing::PlayingState,
    types::{Card, GameContract, GameContractData, GameContractKind},
};

#[derive(Debug)]
pub struct ChoosingCardsState {
    contract_bid: GameContract,
}

impl ChoosingCardsState {
    pub fn new(contract_bid: GameContract) -> Self {
        Self { contract_bid }
    }
}

impl Game<ChoosingCardsState> {
    pub fn validate(&self, action: &GameAction) -> bool {
        match &action.kind {
            GameActionKind::ChooseCards(choice) => self.validate_choose_cards(choice),
            _ => false,
        }
    }

    fn validate_choose_cards(&self, choice: &CardChoice) -> bool {
        if choice.take_cards.len() == choice.discard_cards.len() {
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
        GameState::ChoosingContract(self.into())
    }

    pub fn contract_bid(&self) -> GameContract {
        self.state.contract_bid
    }
}

impl From<Game<ChoosingCardsState>> for Game<ChoosingContractState> {
    fn from(prev: Game<ChoosingCardsState>) -> Self {
        Self {
            state: ChoosingContractState {
                contract_bid: prev.state.contract_bid,
            },
            first: prev.first,
            turn: prev.turn,
            cards: prev.cards,
            score: prev.score,
        }
    }
}

#[derive(Debug)]
pub struct ChoosingContractState {
    contract_bid: GameContract,
}

impl Game<ChoosingContractState> {
    pub fn validate(&self, action: &GameAction) -> bool {
        match &action.kind {
            GameActionKind::ChooseContract(contract) => *contract > self.state.contract_bid,
            _ => false,
        }
    }

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        match action.kind {
            GameActionKind::ChooseContract(contract) => Ok(self.choose_contract(contract)),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn choose_contract(mut self, contract: GameContract) -> GameState {
        self.state.contract_bid = contract;

        GameState::RespondingToContract(self.into())
    }
}

impl From<Game<ChoosingContractState>> for Game<RespondingToContractState> {
    fn from(prev: Game<ChoosingContractState>) -> Self {
        let next_turn = turn_inc(prev.turn);

        Self {
            state: RespondingToContractState {
                contract: GameContractData {
                    value: prev.state.contract_bid,
                    kind: GameContractKind::Bid,
                },
                declarer_ind: prev.turn,
                player_responses: Default::default(),
            },
            first: prev.first,
            turn: next_turn,
            cards: prev.cards,
            score: prev.score,
        }
    }
}

#[derive(Debug)]
pub struct RespondingToContractState {
    contract: GameContractData,
    declarer_ind: usize,
    player_responses: [PlayerResponseState; 3],
}

impl RespondingToContractState {
    pub fn new(contract: GameContractData, declarer_ind: usize) -> Self {
        Self {
            contract,
            declarer_ind,
            player_responses: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PlayerResponseState {
    #[default]
    NoResponse,
    Accepted,
    Rejected,
    Caller,
    Called,
}

impl Game<RespondingToContractState> {
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

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        use GameActionKind::*;
        match action.kind {
            AcceptContract => Ok(self.accept_contract()),
            RejectContract => Ok(self.reject_contract()),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn accept_contract(mut self) -> GameState {
        self.state.player_responses[self.turn] = PlayerResponseState::Accepted;

        self.to_next()
    }

    fn reject_contract(mut self) -> GameState {
        self.state.player_responses[self.turn] = PlayerResponseState::Rejected;

        self.to_next()
    }

    fn to_next(self) -> GameState {
        let number_of_responses = self.count_responses();

        if number_of_responses < 2 {
            self.to_next_respond_to_contract_state()
        } else {
            self.to_help_or_contre_state()
        }
    }

    fn to_next_respond_to_contract_state(mut self) -> GameState {
        self.turn = turn_inc(self.turn);

        GameState::RespondingToContract(self)
    }

    fn to_help_or_contre_state(self) -> GameState {
        GameState::HelpOrContreToContract(self.into())
    }

    fn count_responses(&self) -> usize {
        self.state
            .player_responses
            .iter()
            .filter(|&&r| r != PlayerResponseState::NoResponse)
            .count()
    }
}

impl From<Game<RespondingToContractState>> for Game<HelpOrContreToContractState> {
    fn from(prev: Game<RespondingToContractState>) -> Self {
        let next_turn = turn_inc(prev.turn);

        Self {
            state: HelpOrContreToContractState {
                contract: prev.state.contract,
                declarer_ind: prev.state.declarer_ind,
                player_responses: prev.state.player_responses,
            },
            first: prev.first,
            turn: next_turn,
            cards: prev.cards,
            score: prev.score,
        }
    }
}

#[derive(Debug)]
pub struct HelpOrContreToContractState {
    contract: GameContractData,
    declarer_ind: usize,
    player_responses: [PlayerResponseState; 3],
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
        self.state.player_responses[teammate_ind] == PlayerResponseState::Rejected
    }

    fn validate_declare_contre(&self, player_ind: usize) -> bool {
        assert_ids_differ(self.state.declarer_ind, player_ind);

        true
    }

    fn validate_pass_help_contre(&self) -> bool {
        true
    }

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        match &action.kind {
            GameActionKind::CallForHelp => Ok(self.call_for_help()),
            GameActionKind::DeclareContre => Ok(self.declare_contre()),
            GameActionKind::PassHelpContre => Ok(self.pass_help_contre()),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn call_for_help(mut self) -> GameState {
        self.state.player_responses[self.turn] = PlayerResponseState::Caller;
        let called_ind = get_third_ind(self.turn, self.state.declarer_ind);
        self.state.player_responses[called_ind] = PlayerResponseState::Called;

        GameState::Playing(self.into())
    }

    fn declare_contre(self) -> GameState {
        GameState::ContreDeclared(self.into())
    }

    fn pass_help_contre(mut self) -> GameState {
        let next_turn = turn_inc(self.turn);
        if next_turn == self.state.declarer_ind {
            GameState::Playing(self.into())
        } else {
            self.turn = next_turn;
            GameState::HelpOrContreToContract(self)
        }
    }
}

impl From<Game<HelpOrContreToContractState>> for Game<ContreDeclaredState> {
    fn from(prev: Game<HelpOrContreToContractState>) -> Game<ContreDeclaredState> {
        Self {
            state: ContreDeclaredState {
                contract: prev.state.contract,
                declarer_ind: prev.state.declarer_ind,
                contre_level: ContreLevel::Contre,
                player_responses: prev.state.player_responses,
            },
            first: prev.first,
            turn: prev.state.declarer_ind,
            cards: prev.cards,
            score: prev.score,
        }
    }
}

impl From<Game<HelpOrContreToContractState>> for Game<PlayingState> {
    fn from(prev: Game<HelpOrContreToContractState>) -> Game<PlayingState> {
        let turn = prev
            .state
            .contract
            .value
            .first_to_play(prev.first, prev.state.declarer_ind);

        Self {
            state: PlayingState::new(
                prev.state.contract,
                prev.state.declarer_ind,
                prev.state.player_responses,
            ),
            first: prev.first,
            turn: turn,
            cards: prev.cards,
            score: prev.score,
        }
    }
}

#[derive(Debug)]
pub struct ContreDeclaredState {
    contract: GameContractData,
    declarer_ind: usize,
    contre_level: ContreLevel,
    player_responses: [PlayerResponseState; 3],
}

impl ContreDeclaredState {
    fn contre_declarer_ind(&self) -> usize {
        self.player_responses
            .iter()
            .position(|&x| x == PlayerResponseState::NoResponse)
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ContreLevel {
    Contre,
    Recontre,
    Subcontre,
    FuckYouContre,
}

impl ContreLevel {
    fn next(&self) -> Self {
        match self {
            ContreLevel::Contre => ContreLevel::Recontre,
            ContreLevel::Recontre => ContreLevel::Subcontre,
            ContreLevel::Subcontre => ContreLevel::FuckYouContre,
            ContreLevel::FuckYouContre => ContreLevel::FuckYouContre,
        }
    }
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
        let last_declarer_id = self.last_declarer_id();

        assert_ids_differ(last_declarer_id, player_ind);

        true
    }

    fn validate_pass_help_contre(&self) -> bool {
        true
    }

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        match action.kind {
            GameActionKind::DeclareContre => Ok(self.apply_declare_contre()),
            GameActionKind::PassHelpContre => Ok(self.apply_pass_help_contre()),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn apply_declare_contre(mut self) -> GameState {
        self.turn = self.next_declarer_id();
        self.state.contre_level = self.state.contre_level.next();

        if self.state.contre_level != ContreLevel::FuckYouContre {
            GameState::ContreDeclared(self)
        } else {
            GameState::Playing(self.into())
        }
    }

    fn last_declarer_id(&self) -> usize {
        match &self.state.contre_level {
            ContreLevel::Contre | ContreLevel::Subcontre => self.state.declarer_ind,
            ContreLevel::Recontre | ContreLevel::FuckYouContre => self.state.contre_declarer_ind(),
        }
    }

    fn next_declarer_id(&self) -> usize {
        self.last_declarer_id()
    }

    fn apply_pass_help_contre(self) -> GameState {
        GameState::Playing(self.into())
    }
}

impl From<Game<ContreDeclaredState>> for Game<PlayingState> {
    fn from(prev: Game<ContreDeclaredState>) -> Self {
        let turn = prev
            .state
            .contract
            .value
            .first_to_play(prev.first, prev.state.declarer_ind);

        Self {
            state: PlayingState::new(
                prev.state.contract,
                prev.state.declarer_ind,
                prev.state.player_responses,
            ),
            first: prev.first,
            turn: turn,
            cards: prev.cards,
            score: prev.score,
        }
    }
}

fn assert_ids_differ(last_declarer_ind: usize, player_ind: usize) {
    debug_assert!(
        last_declarer_ind != player_ind,
        "Player should not be able to respond to his own contre"
    );
}
