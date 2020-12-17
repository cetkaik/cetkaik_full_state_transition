#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum Season {
    ///春, Spring
    Iei2,
    ///夏, Summer
    Xo1,
    ///秋, Autumn
    Kat2,
    ///冬, Winter
    Iat1,
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
pub struct ExistenceOfHandNotResolved {
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

impl StateC {
    pub fn piece_at_flying_piece_src(&self) -> absolute::Piece {
        *self
            .f
            .board
            .get(&self.flying_piece_src)
            .expect("Invalid StateC: at flying_piece_src there is no piece")
    }
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

pub enum NormalMove {
    NonTamMoveSrcDst {
        src: absolute::Coord,
        dest: absolute::Coord,
        /* is_water_entry_ciurl: bool, */
    },
    NonTamMoveSrcStepDstFinite {
        src: absolute::Coord,
        step: absolute::Coord,
        dest: absolute::Coord,
        /* is_water_entry_ciurl: bool, */
    },
    NonTamMoveFromHand {
        color: cetkaik_core::Color,
        prof: cetkaik_core::Profession,
        dest: absolute::Coord,
    },
    TamMoveNoStep {
        src: absolute::Coord,
        first_dest: absolute::Coord,
        second_dest: absolute::Coord,
    },
    TamMoveStepsDuringFormer {
        src: absolute::Coord,
        step: absolute::Coord,
        first_dest: absolute::Coord,
        second_dest: absolute::Coord,
    },
    TamMoveStepsDuringLatter {
        src: absolute::Coord,
        step: absolute::Coord,
        first_dest: absolute::Coord,
        second_dest: absolute::Coord,
    },
}

pub struct InfAfterStep {
    pub color: cetkaik_core::Color,
    pub prof: cetkaik_core::Profession,
    pub src: absolute::Coord,
    pub step: absolute::Coord,
    pub planned_direction: absolute::Coord,
}
pub struct AfterHalfAcceptance {
    /// None: hands over the turn to the opponent
    /// None は（投げ棒の出目が気に入らなかったために）パスして相手に手番を渡すことを表す
    pub dest: Option<absolute::Coord>,
}

pub fn apply_normal_move(
    old_state: &StateA,
    msg: NormalMove,
) -> Probabilistic<ExistenceOfHandNotResolved> {
    unimplemented!()
}

pub fn apply_inf_after_step(old_state: &StateA, msg: InfAfterStep) -> Probabilistic<StateC> {
    unimplemented!()
}

pub fn apply_after_half_acceptance(
    old_state: &StateC,
    msg: AfterHalfAcceptance,
) -> Probabilistic<ExistenceOfHandNotResolved> {
    let StateC {
        flying_piece_src: src,
        flying_piece_step: step,
        ..
    } = *old_state;

    if let Some(msgdest) = msg.dest {
        let piece = old_state.piece_at_flying_piece_src();
        if absolute::is_water(src) || piece.has_prof(cetkaik_core::Profession::Nuak1) {
            /*

                    const {
              hand_is_made,
            } = movePieceFromSrcToDestWhileTakingOpponentPieceIfNeeded(
              game_state,
              src,
              msg.dest,
              room_info.is_IA_down_for_me,
            );
            const final_obj = getLastMove(game_state);
            if (typeof final_obj === "undefined" || !isInfAfterStep(final_obj)) {
              return { legal: false, whyIllegal: "the last move was not InfAfterStep" };
            }

            final_obj.move.finalResult = {
              dest: msg.dest,
            };

            final_obj.status = hand_is_made ? "not yet" : null;

            const ans: Ret_AfterHalfAcceptance = {
              legal: true,
              dat: {
                waterEntryHappened: false,
              },
            };

            ifStepTamEditScore(game_state, step, room_info);
            return ans;

                    */
            unimplemented!()
        }

        if absolute::is_water(msgdest) {
            unimplemented!("if step tam edit score")
        }

        unimplemented!()
    } else {
        // the only possible side effect is that Stepping Tam might
        // modify the score. Water entry cannot fail,
        // since the piece has not actually moved.
        // 唯一ありえる副作用は、撃皇で点が減っている可能性があるということ
        // パスが発生した以上、駒の動きは実際には発生していないので、
        // 入水判定は発生していない。

        unimplemented!("if step tam edit score")
    }
}

/// `ExistenceOfHandNotResolved` を `resolve` でこの型に変換することによって、
/// 「役は発生しなかったぞ」 vs.
/// 「役は発生しており、したがって
/// * 再行ならこの `StateA` に至る
/// * 終季ならこの `Probabilistic<StateA>` に至る
/// （どちらが先手になるかは鯖のみぞ知るので `Probabilistic`）
/// 」のどちらであるかを知ることができる。
/// 撃皇が役を構成するかどうかによってここの処理は変わってくるので、
/// `Config` が要求されることになる。
pub enum ExistenceOfHandResolved {
    NeitherTymokNorTaxot(StateA),
    HandExists {
        if_tymok: StateA,
        if_taxot: Probabilistic<StateA>,
    },
}

pub struct Config {
    pub step_tam_is_a_hand: bool,
}

impl ExistenceOfHandNotResolved {
    pub fn resolve(self, config: Config) -> ExistenceOfHandResolved {
        unimplemented!()
    }
}
