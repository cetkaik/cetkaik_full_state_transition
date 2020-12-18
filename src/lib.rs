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
    whose_turn: absolute::Side,
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
    whose_turn: absolute::Side,
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

    pub fn piece_at_flying_piece_step(&self) -> absolute::Piece {
        *self
            .f
            .board
            .get(&self.flying_piece_step)
            .expect("Invalid StateC: at flying_piece_step there is no piece")
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
/*
struct HandIsMade {
    hand_is_made: bool,
}

fn movePieceFromSrcToDestWhileTakingOpponentPieceIfNeeded(
    game_state: GameState,
    src: absolute::Coord,
    dest: absolute::Coord,
    is_IA_down_for_me: bool,
) -> Result<HandIsMade, &'static str> {
    let piece = setPiece(game_state, src, None).ok_or("src does not contain a piece")?;
    let maybe_taken = setPiece(game_state, dest, piece);
    if let Ok(taken) = maybe_taken {
        if is_IA_down_for_me {
            if !isNonTam2PieceNonIAOwner(taken) {
                return Err("tried to take either an ally or tam2");
            }
            let old_state = calculateHandsAndScore(game_state.f.hop1zuo1OfIAOwner);
            addToHop1Zuo1OfIAOwner(game_state, taken);
            let new_state = calculateHandsAndScore(game_state.f.hop1zuo1OfIAOwner);
            return Ok(HandIsMade {
                hand_is_made: new_state.score != old_state.score,
            });
        } else {
            if !isNonTam2PieceIAOwner(taken) {
                return Err("tried to take either an ally or tam2");
            }
            let old_state = calculateHandsAndScore(game_state.f.hop1zuo1OfNonIAOwner);
            addToHop1Zuo1OfNonIAOwner(game_state, taken);
            let new_state = calculateHandsAndScore(game_state.f.hop1zuo1OfNonIAOwner);
            return Ok(HandIsMade {
                hand_is_made: new_state.score != old_state.score,
            });
        }
    }
    return Ok(HandIsMade {
        hand_is_made: false,
    });
}*/

pub fn apply_after_half_acceptance(
    old_state: &StateC,
    msg: AfterHalfAcceptance,
) -> Result<Probabilistic<ExistenceOfHandNotResolved>, &'static str> {
    let StateC {
        flying_piece_src: src,
        flying_piece_step: step,
        ..
    } = *old_state;

    if let Some(msgdest) = msg.dest {
        let piece = old_state.piece_at_flying_piece_src();

        // Either the piece started from within water, or the piece is a Vessel; no need to
        // cast sticks to tell whether you can enter the water.
        // 出発地点が皇水であるか、移動している駒が船であるため、いかなる条件でも入水判定が不要。
        if absolute::is_water(src) || piece.has_prof(cetkaik_core::Profession::Nuak1) {
            match old_state.f.board.get(&msgdest) {
                None => {
                    // no piece is to be captured

                    // 入水判定が絶対にないので確率は1
                    // succeeds with probability 1
                }

                Some(cetkaik_core::absolute::Piece::Tam2) => return Err("cannot capture a Tam2"),

                Some(&cetkaik_core::absolute::Piece::NonTam2Piece {
                    color: captured_piece_color,
                    prof: captured_piece_prof,
                    side: captured_piece_side,
                }) => {
                    if old_state.whose_turn == captured_piece_side {
                        return Err("cannot capture your own piece");
                    }

                    // 入水判定が絶対にないので確率は1
                    // succeeds with probability 1

                    let new_state: StateA = unimplemented!();

                    return Ok(Probabilistic::pure(ExistenceOfHandNotResolved {
                        previous_a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
                        previous_ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
                        kut2tam2_happened: old_state.piece_at_flying_piece_step().is_tam2(),
                        state: new_state
                    }));
                }
            }
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
