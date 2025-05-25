use super::{
    actions::{GameAction, GameActionKind},
    choosing::ChoosingCardsState,
    game::{CardsInPlay, Game, GameError, GameState, PlayerScore},
    types::GameContract,
};

pub struct BiddingState {
    bid: Option<Bid>,
    can_steal_bid: bool,
    player_states: [PlayerBidState; 3],
}

pub struct Bid {
    value: GameContract,
    bidder_ind: usize,
}

impl BiddingState {
    fn where_next(&self) -> NextAfterBidding {
        let mut passed_players = 0;
        for i in 0..3 {
            match self.player_states[i] {
                PlayerBidState::PassedBid => passed_players += 1,
                PlayerBidState::NoBid => return NextAfterBidding::Continue,
                _ => {}
            };
        }

        debug_assert!(passed_players <= 3);

        match passed_players {
            0 | 1 => NextAfterBidding::Continue,
            2 => NextAfterBidding::ToChoosingCards,
            _ => NextAfterBidding::ToNewHand,
        }
    }
}

enum NextAfterBidding {
    Continue,
    ToChoosingCards,
    ToNewHand,
}

#[derive(Default, PartialEq, PartialOrd)]
enum PlayerBidState {
    #[default]
    NoBid,
    Bid(GameContract),
    PassedBid,
}

impl Game<BiddingState> {
    pub fn new(first: usize, score: [PlayerScore; 3]) -> Self {
        Game {
            state: BiddingState {
                bid: None,
                can_steal_bid: false,
                player_states: Default::default(),
            },
            first,
            turn: first,
            cards: CardsInPlay::deal_random(),
            score,
        }
    }

    pub fn validate(&self, action: &GameAction) -> bool {
        match action.kind {
            GameActionKind::Bid | GameActionKind::PassBid => self.validate_bid(action.player_ind),
            GameActionKind::ClaimNoBid => self.validate_claim_nobid(action.player_ind),
            _ => false,
        }
    }

    fn validate_bid(&self, player_ind: usize) -> bool {
        if let Some(bid) = &self.state.bid {
            debug_assert!(
                bid.bidder_ind != player_ind,
                "Player should not be able to respond to his own bid"
            );
        }

        true
    }

    fn validate_claim_nobid(&self, player_ind: usize) -> bool {
        self.state.player_states[player_ind] == PlayerBidState::NoBid
    }

    pub fn apply(self, action: GameAction) -> Result<GameState, GameError> {
        match action.kind {
            GameActionKind::Bid => Ok(self.bid(action.player_ind)),
            GameActionKind::PassBid => Ok(self.pass_bid(action.player_ind)),
            GameActionKind::ClaimNoBid => Ok(self.claim_no_bid(action.player_ind)),
            _ => Err(GameError::InvalidAction),
        }
    }

    fn bid(mut self, bidder_ind: usize) -> GameState {
        let bid_value = self.next_bid_value();

        self.state.can_steal_bid = false;
        self.state.player_states[bidder_ind] = PlayerBidState::Bid(bid_value);

        self.to_next(Bid {
            value: bid_value,
            bidder_ind,
        })
    }

    fn next_bid_value(&self) -> GameContract {
        if let Some(current_bid) = &self.state.bid {
            self.next_after_existing_bid(current_bid.value)
        } else {
            GameContract::Spades
        }
    }

    fn next_after_existing_bid(&self, current_value: GameContract) -> GameContract {
        if self.state.can_steal_bid {
            current_value
        } else {
            current_value.next()
        }
    }

    fn is_circle_completed(&self) -> bool {
        self.turn == self.first
    }

    fn pass_bid(self, player_ind: usize) -> GameState {
        todo!()
    }

    fn claim_no_bid(self, player_ind: usize) -> GameState {
        todo!()
    }

    fn to_next(self, bid: Bid) -> GameState {
        let where_next = self.state.where_next();
        match where_next {
            NextAfterBidding::Continue => self.to_next_bidding_state(bid),
            NextAfterBidding::ToChoosingCards => self.to_choosing_cards(),
            NextAfterBidding::ToNewHand => self.to_new_hand(),
        }
    }

    fn to_next_bidding_state(mut self, bid: Bid) -> GameState {
        self.turn = self.next_turn();

        if self.is_circle_completed() {
            /*
               Comment of defeat:
               Whenever a circle completes, the next player to bid will not increase the bid. Instead,
               he gets to "steal" the current bid value.

               Example 1:
               2 -> 3 -> 4 -> Steal 4 -> 5

               Example 2:
               2 -> 3 -> 4 -> Pass -> Steal 4 -> 5
            */
            self.state.can_steal_bid = true;
        }

        self.state.bid = Some(bid);

        GameState::Bidding(self)
    }

    fn to_choosing_cards(self) -> GameState {
        let bid = self.state.bid.unwrap();

        GameState::ChoosingCards(<Game<ChoosingCardsState>>::new(
            self.first,
            bid.bidder_ind,
            self.cards,
            self.score,
            bid.value,
        ))
    }

    fn to_new_hand(&self) -> GameState {
        GameState::Bidding(<Game<BiddingState>>::new(
            self.first + 1,
            Default::default(),
        ))
    }

    fn next_turn(&self) -> usize {
        let mut turn = self.turn_inc();
        for _ in 0..3 {
            if self.state.player_states[turn] != PlayerBidState::PassedBid {
                return turn;
            }

            turn = self.turn_inc();
        }

        panic!("There should be at least one more person who has not passed bid.");
    }
}

pub struct NoBidClaimState {
    claimed: [bool; 3],
    already_bid: [bool; 3],
}

impl Game<NoBidClaimState> {
    pub fn validate(&self, action: &GameAction) -> bool {
        match &action.kind {
            GameActionKind::ClaimNoBid => !self.state.already_bid[action.player_ind],
            _ => false,
        }
    }
}

pub struct NoBidChoiceState {
    contract: GameContract,
    declarer_ind: usize,
}

impl Game<NoBidChoiceState> {
    pub fn validate(&self, action: &GameAction) -> bool {
        match &action.kind {
            GameActionKind::ChooseNoBidContract(contract) => *contract > self.state.contract,
            _ => false,
        }
    }
}
