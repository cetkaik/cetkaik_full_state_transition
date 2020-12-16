#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum Season {
    Iei2, //Spring
    Xo1,  //Summer
    Kat2, // Autumn
    Iat1, // Winter
}

mod prob_density;

impl Season {
    pub fn next(self) -> Option<Self> {
        match self {
            Season::Iei2 => Some(Season::Xo1),
            Season::Xo1 => Some(Season::Kat2),
            Season::Kat2 => Some(Season::Iat1),
            Season::Iat1 => None,
        }
    }

    pub fn to_index(self) -> usize {
        match self {
            Season::Iei2 => 0,
            Season::Xo1 => 1,
            Season::Kat2 => 2,
            Season::Iat1 => 3,
        }
    }
}

use cetkaik_core::absolute;

/// 一番普通の状態。定常状態。
#[derive(Clone, Debug)]
pub struct StateA {
    f: absolute::Field,
    tam_itself_is_tam_hue: bool,
    is_ia_owner_s_turn: bool,
    season: Season,
    ia_owner_s_score: i32,
    rate: Rate,
    tam_has_moved_previously: bool,
}

/// 入水判定も終わり、駒を完全に動かし終わった。
/// しかしながら、「役が存在していて再行・終季をユーザーに訊く」を
/// 発生させるか否かをまだ解決していない。
/// そんな状態。
#[derive(Clone, Debug)]
pub struct StateB {
    state: StateA,
    previous_a_side_hop1zuo1: Vec<absolute::NonTam2Piece>,
    previous_ia_side_hop1zuo1: Vec<absolute::NonTam2Piece>,
    kut2tam2_happened: bool,
}

/// 踏越え後の無限移動をユーザーが行い、それに対して裁で判定した後の状態。
/// 裁を見て、ユーザーは最終的な移動場所をCに対しこれから送りつける。
#[derive(Clone, Debug)]
pub struct StateC {
    f: absolute::Field,
    tam_itself_is_tam_hue: bool,
    is_ia_owner_s_turn: bool,
    flying_piece_src: absolute::Coord,
    flying_piece_step: absolute::Coord,
    season: Season,
    ia_owner_s_score: i32,
    rate: Rate,
}

#[derive(Clone, Copy, Debug)]
pub enum Rate {
    /*
     * Theoretically speaking, it is necessary to distinguish x32 and x64
     * because it is possible to score 1 point (3+3-5).
     * Not that it will ever be of use in any real situation.
     */
    X1,
    X2,
    X4,
    X8,
    X16,
    X32,
    X64,
}

use prob_density::Probabilistic;

impl Rate {
    pub fn next(self) -> Option<Self> {
        match self {
            Rate::X1 => Some(Rate::X2),
            Rate::X2 => Some(Rate::X4),
            Rate::X4 => Some(Rate::X8),
            Rate::X8 => Some(Rate::X16),
            Rate::X16 => Some(Rate::X32),
            Rate::X32 => Some(Rate::X64),
            Rate::X64 => Some(Rate::X64),
        }
    }
}

pub type NormalMove = ();
pub type InfAfterStep = ();
pub type AfterHalfAcceptance = ();

pub fn apply_normal_move(old_state: &StateA, msg: NormalMove) -> Probabilistic<StateB> {
    unimplemented!()
}

pub fn apply_inf_after_step(old_state: &StateA, msg: InfAfterStep) -> Probabilistic<StateC> {
    unimplemented!()
}

pub fn apply_after_half_acceptance(old_state: &StateC, msg: AfterHalfAcceptance) -> Probabilistic<StateB> {
    unimplemented!()
}

enum Foo {
    NeitherTymokNorTaxot(StateA),
    TymokOrTaxot{
        if_tymok: StateA,
        if_taxot: Probabilistic<StateA>
    }
}

impl Into<Foo> for StateB {
    fn into(self) -> Foo {
        unimplemented!()
    }
}
