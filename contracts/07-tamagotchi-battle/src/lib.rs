#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{collections::BTreeMap, prelude::*, ActorId, MessageId, ReservationId};

pub type TamagotchiId = ActorId;
pub type PairId = u8;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct BattleMetadata;

impl Metadata for BattleMetadata {
    type Init = ();
    type Handle = InOut<BattleAction, BattleEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<Battle>;
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Battle {
    pub admins: Vec<ActorId>,
    pub players: BTreeMap<ActorId, Player>,
    pub players_ids: Vec<ActorId>,
    pub current_players: Vec<ActorId>,
    pub state: BattleState,
    pub current_winner: ActorId,
    pub pairs: BTreeMap<PairId, Pair>,
    pub players_to_pairs: BTreeMap<ActorId, Vec<PairId>>,
    pub completed_games: u8,
    pub reservations: BTreeMap<ActorId, ReservationId>,
}

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Player {
    pub owner: ActorId,
    pub name: String,
    pub date_of_birth: u64,
    pub tmg_id: TamagotchiId,
    pub defence: u16,
    pub power: u16,
    pub health: u16,
    pub color: String,
    pub victories: u32,
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Move {
    Attack,
    Defence,
}

#[derive(Default, Debug, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Pair {
    pub owner_ids: Vec<ActorId>,
    pub tmg_ids: Vec<ActorId>,
    pub moves: Vec<Option<Move>>,
    pub rounds: u8,
    pub game_is_over: bool,
    pub winner: ActorId,
    pub move_deadline: u64,
    pub msg_id: MessageId,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleState {
    #[default]
    Registration,
    GameIsOn,
    WaitNextRound,
    GameIsOver,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleAction {
    StartRegistration,
    Register {
        tmg_id: TamagotchiId,
    },
    MakeMove {
        pair_id: PairId,
        tmg_move: Move,
    },
    StartBattle,
    AddAdmin(ActorId),
    CheckIfMoveMade {
        pair_id: PairId,
        tmg_id: Option<TamagotchiId>,
    },
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BattleEvent {
    RegistrationStarted,
    Registered { tmg_id: TamagotchiId },
    MoveMade,
    GoToWaitingState,
    GameIsOver,
    InfoUpdated,
    NewGame,
    BattleStarted,
    RoundResult((PairId, u16, u16, Option<Move>, Option<Move>)),
    NewRound,
    AdminAdded,
}

const MAX_POWER: u16 = 10_000;
const MAX_RANGE: u16 = 7_000;
const MIN_RANGE: u16 = 3_000;
const HEALTH: u16 = 2_500;
const MAX_PARTICIPANTS: u8 = 50;
const MAX_STEPS_IN_ROUND: u8 = 5;
const COLORS: [&str; 6] = ["Green", "Red", "Blue", "Purple", "Orange", "Yellow"];
const TIME_FOR_MOVE: u32 = 20;
const GAS_AMOUNT: u64 = 10_000_000_000;
const RESERVATION_AMOUNT: u64 = 200_000_000_000;
const RESERVATION_DURATION: u32 = 86_400;

#[gstd::async_main]
async fn main() {
    let action: BattleAction = msg::load().expect("Unable to decode `BattleAction`");
    let battle = unsafe { BATTLE.get_or_insert(Default::default()) };
    match action {
        BattleAction::StartRegistration => battle.start_registration(),
        BattleAction::Register { tmg_id } => battle.register(&tmg_id).await,
        BattleAction::MakeMove { pair_id, tmg_move } => battle.make_move(pair_id, tmg_move),
        BattleAction::StartBattle => battle.start_battle(),
        BattleAction::AddAdmin(new_admin) => battle.add_admin(&new_admin),
        BattleAction::CheckIfMoveMade { pair_id, tmg_id } => {
            battle.check_if_move_made(pair_id, tmg_id)
        }
    }
}

#[no_mangle]
unsafe extern fn init() {
    let battle = Battle {
        admins: vec![msg::source()],
        ..Default::default()
    };
    BATTLE = Some(battle);
}

pub async fn get_tmg_info(tmg_id: &ActorId) -> (ActorId, String, u64) {
    let reply: TmgReply = msg::send_for_reply_as(*tmg_id, TmgAction::TmgInfo, 0, 0)
        .expect("Error in sending a message `TmgAction::TmgInfo")
        .await
        .expect("Unable to decode TmgReply");
    if let TmgReply::TmgInfo {
        owner,
        name,
        date_of_birth,
    } = reply
    {
        (owner, name, date_of_birth)
    } else {
        panic!("Wrong received message");
    }
}

static mut SEED: u8 = 0;

pub fn get_random_value(range: u8) -> u8 {
    if range == 0 {
        return 0;
    }
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    random[0] % range
}

