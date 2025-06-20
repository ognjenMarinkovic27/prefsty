use crate::http::{
    ApiContext,
    extractors::AuthUser,
    repos::{
        game::GameRepo,
        model,
        model::{GameId, UserId, UserSafeIdx},
    },
};
use axum::{
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, Utf8Bytes, WebSocket},
    },
    response::Response,
};
use dashmap::DashMap;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use prefsty::core::{
    actions::{GameAction, GameActionKind},
    bidding::{
        bidding::BiddingState,
        no_bid::{NoBidChoiceState, NoBidClaimState},
    },
    choosing::{
        ChoosingCardsState, ChoosingContractState, ContreDeclaredState,
        HelpOrContreToContractState, RespondingToContractState,
    },
    game::{Game, GameState, PlayerScore, Refas},
    playing::PlayingState,
    types::Card,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

pub type ClientMap = DashMap<(GameId, UserId), UnboundedSender<Message>>;

pub async fn handler(
    ws: WebSocketUpgrade,
    user: AuthUser,
    Path(game_id): Path<GameId>,
    State(state): State<ApiContext>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, user, game_id, state))
}

async fn handle_socket(socket: WebSocket, user: AuthUser, game_id: GameId, state: ApiContext) {
    let (ws_tx, ws_rx) = socket.split();
    let (tx, rx) = unbounded_channel::<Message>();
    let user_id = user.user_id;
    state.clients.insert((game_id, user_id), tx.clone());

    let write_task = tokio::spawn(write(ws_tx, rx));

    let read_task = {
        let state = state.clone();
        tokio::spawn(read(ws_rx, tx, game_id, user_id, state))
    };

    tokio::select! {
        _ = write_task => {},
        _ = read_task => {},
    }

    state.clients.remove(&(game_id, user_id));
}

async fn read(
    mut ws_rx: SplitStream<WebSocket>,
    tx: UnboundedSender<Message>,
    game_id: GameId,
    user_id: UserId,
    state: ApiContext,
) {
    while let Some(msg) = ws_rx.next().await {
        let msg_bytes = if let Ok(Message::Text(bytes)) = msg {
            bytes
        } else {
            // client disconnected
            return;
        };

        // process and serialize response
        if let Err(err) = handle_message(msg_bytes, user_id, game_id, &state).await {
            let err_msg = serde_json::to_string(&OutgoingMessage {
                kind: OutgoingMessageKind::Error(err.to_string()),
            })
            .unwrap();

            if tx.send(err_msg.into()).is_err() {
                // client disconnected
                return;
            }
        }
    }
}

async fn write(mut ws_tx: SplitSink<WebSocket, Message>, mut rx: UnboundedReceiver<Message>) {
    while let Some(msg) = rx.recv().await {
        if ws_tx.send(msg).await.is_err() {
            // sink closed, stop writing
            break;
        }
    }
}

#[derive(Serialize, Deserialize)]
struct IncomingMessage {
    kind: IncomingMessageKind,
}

#[derive(Serialize, Deserialize)]
enum IncomingMessageKind {
    Game(GameActionKind),
    Sync,
}

#[derive(Serialize)]
struct OutgoingMessage<'a> {
    kind: OutgoingMessageKind<'a>,
}

#[derive(Serialize)]
enum ClientGameStateView<'a> {
    Bidding(ClientGameView<'a, BiddingState>),
    NoBidPlayClaim(ClientGameView<'a, NoBidClaimState>),
    NoBidPlayChoice(ClientGameView<'a, NoBidChoiceState>),
    ChoosingCards(ClientGameView<'a, ChoosingCardsState>),
    ChoosingContract(ClientGameView<'a, ChoosingContractState>),
    RespondingToContract(ClientGameView<'a, RespondingToContractState>),
    HelpOrContreToContract(ClientGameView<'a, HelpOrContreToContractState>),
    ContreDeclared(ClientGameView<'a, ContreDeclaredState>),
    Playing(ClientGameView<'a, PlayingState>),
}

impl<'a> ClientGameStateView<'a> {
    pub fn from_state_for_player(state: &'a GameState, player: usize) -> Self {
        use prefsty::core::game::GameState::*;
        match state {
            Bidding(game) => {
                ClientGameStateView::Bidding(ClientGameView::from_state_for_player(&game, player))
            }
            NoBidPlayClaim(game) => ClientGameStateView::NoBidPlayClaim(
                ClientGameView::from_state_for_player(&game, player),
            ),
            NoBidPlayChoice(game) => ClientGameStateView::NoBidPlayChoice(
                ClientGameView::from_state_for_player(&game, player),
            ),
            ChoosingCards(game) => ClientGameStateView::ChoosingCards(
                ClientGameView::from_state_for_player(&game, player),
            ),
            ChoosingContract(game) => ClientGameStateView::ChoosingContract(
                ClientGameView::from_state_for_player(&game, player),
            ),
            RespondingToContract(game) => ClientGameStateView::RespondingToContract(
                ClientGameView::from_state_for_player(&game, player),
            ),
            HelpOrContreToContract(game) => ClientGameStateView::HelpOrContreToContract(
                ClientGameView::from_state_for_player(&game, player),
            ),
            ContreDeclared(game) => ClientGameStateView::ContreDeclared(
                ClientGameView::from_state_for_player(&game, player),
            ),
            Playing(game) => {
                ClientGameStateView::Playing(ClientGameView::from_state_for_player(&game, player))
            }
        }
    }
}

#[derive(Serialize)]
struct ClientGameView<'a, S> {
    pub state: &'a S,
    pub first: usize,
    pub turn: usize,
    pub hand: &'a Vec<Card>,
    pub score: &'a [PlayerScore; 3],
    pub refas: &'a Refas,
}

impl<'a, S> ClientGameView<'a, S> {
    pub fn from_state_for_player(game: &'a Game<S>, player: usize) -> Self {
        Self {
            state: &game.state,
            first: game.first,
            turn: game.turn,
            hand: &game.cards.hands[player],
            score: &game.score,
            refas: &game.refas,
        }
    }
}

#[derive(Serialize)]
enum OutgoingMessageKind<'a> {
    State(ClientGameStateView<'a>),
    Error(String),
}

async fn handle_message(
    bytes: Utf8Bytes,
    user_id: UserId,
    game_id: GameId,
    state: &ApiContext,
) -> anyhow::Result<()> {
    let m: IncomingMessage = serde_json::from_str(bytes.as_str())?;

    if let IncomingMessageKind::Game(action) = m.kind {
        let game_repo: &GameRepo = &state.game_repo;
        let mut game: model::Game = game_repo.get_by_id(game_id).await?;
        let joined: Vec<UserSafeIdx> = game_repo.get_joined_by_game_id(game_id).await?;

        debug_assert!(game.id == game_id, "should be the same, just fetched");

        let player_idx = joined
            .iter()
            .find(|&u| u.id == user_id)
            .ok_or(anyhow::anyhow!("user not in this game"))?
            .idx;

        debug_assert!(
            player_idx >= 0 && player_idx <= 2,
            "index should be in player range",
        );

        game.state = game.state.apply(GameAction {
            player: player_idx as usize,
            kind: action,
        })?;

        game_repo.update(&game).await?;

        let client_game =
            ClientGameStateView::from_state_for_player(&game.state, player_idx as usize);

        let outgoing = serde_json::to_string(&OutgoingMessage {
            kind: OutgoingMessageKind::State(client_game),
        })
        .unwrap();

        let joined_ids = joined.iter().map(|u| u.id);
        for joined_id in joined_ids {
            if let Some(client_tx) = state.clients.get(&(game_id, joined_id)) {
                // if we fail this tough titties, someone else should
                // notice client disconnected
                let _ = client_tx.send(outgoing.clone().into());
            }
        }
    } else {
        return Err(anyhow::anyhow!("not implemented"));
    }

    Ok(())
}
