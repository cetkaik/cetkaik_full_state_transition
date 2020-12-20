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

mod probabilistic;

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
    f: absolute::Field,
    tam_itself_is_tam_hue: bool,
    whose_turn: absolute::Side,
    season: Season,
    ia_owner_s_score: i32,
    rate: Rate,
    tam_has_moved_previously: bool,
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

use probabilistic::Probabilistic;

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

fn apply_tam_move(
    old_state: &StateA,
    src: absolute::Coord,
    first_dest: absolute::Coord,
    second_dest: absolute::Coord,
    step: Option<absolute::Coord>,
) -> Result<Probabilistic<ExistenceOfHandNotResolved>, &'static str> {
    let mut new_field = old_state.f.clone();
    let expect_tam = new_field
        .board
        .remove(&src)
        .ok_or("expected tam2 but found an empty square")?;
    if !expect_tam.is_tam2() {
        return Err("expected tam2 but found a non-tam2 piece");
    }

    if new_field.board.contains_key(&first_dest) {
        return Err("the first destination is already occupied");
    }

    if let Some(st) = step {
        if !new_field.board.contains_key(&st) {
            return Err("the stepping square is empty");
        }
    }

    if new_field.board.contains_key(&second_dest) {
        return Err("the second destination is already occupied");
    }

    new_field.board.insert(second_dest, absolute::Piece::Tam2);
    
    return Ok(Probabilistic::Pure(ExistenceOfHandNotResolved {
        previous_a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
        previous_ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
        kut2tam2_happened: false,
        rate: old_state.rate,
        tam_has_moved_previously: true,
        season: old_state.season,
        tam_itself_is_tam_hue: old_state.tam_itself_is_tam_hue,
        ia_owner_s_score: old_state.ia_owner_s_score,
        whose_turn: old_state.whose_turn,
        f: new_field,
    }));
}

fn apply_nontam_move(
    old_state: &StateA,
    src: absolute::Coord,
    dest: absolute::Coord,
    step: Option<absolute::Coord>,
) -> Result<Probabilistic<ExistenceOfHandNotResolved>, &'static str> {
    unimplemented!()
}

pub fn apply_normal_move(
    old_state: &StateA,
    msg: NormalMove,
) -> Result<Probabilistic<ExistenceOfHandNotResolved>, &'static str> {
    match msg {
        NormalMove::NonTamMoveFromHand { color, prof, dest } => {
            let mut new_field = old_state
                .f
                .find_and_remove_piece_from_hop1zuo1(color, prof, old_state.whose_turn)
                .ok_or("Cannot find the specified piece in the hop1zuo1")?;

            if new_field.board.contains_key(&dest) {
                return Err("The destination is already occupied and hence cannot place a piece from hop1 zuo1");
            }

            new_field.board.insert(
                dest,
                absolute::Piece::NonTam2Piece {
                    color,
                    prof,
                    side: old_state.whose_turn,
                },
            );

            return Ok(Probabilistic::Pure(ExistenceOfHandNotResolved {
                previous_a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
                previous_ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
                kut2tam2_happened: false,
                rate: old_state.rate,
                tam_has_moved_previously: false,
                season: old_state.season,
                tam_itself_is_tam_hue: old_state.tam_itself_is_tam_hue,
                ia_owner_s_score: old_state.ia_owner_s_score,
                whose_turn: old_state.whose_turn,
                f: new_field,
            }));
        }
        NormalMove::TamMoveNoStep {
            src,
            first_dest,
            second_dest,
        } => return apply_tam_move(old_state, src, first_dest, second_dest, None),
        NormalMove::TamMoveStepsDuringFormer {
            src,
            first_dest,
            second_dest,
            step,
        }
        | NormalMove::TamMoveStepsDuringLatter {
            src,
            first_dest,
            second_dest,
            step,
        } => return apply_tam_move(old_state, src, first_dest, second_dest, Some(step)),

        NormalMove::NonTamMoveSrcDst { src, dest } => {
            return apply_nontam_move(old_state, src, dest, None)
        }
        NormalMove::NonTamMoveSrcStepDstFinite { src, step, dest } => {
            return apply_nontam_move(old_state, src, dest, Some(step))
        }
    }
}

pub fn apply_inf_after_step(old_state: &StateA, msg: InfAfterStep) -> Probabilistic<StateC> {
    unimplemented!()
}

fn move_nontam_piece_from_src_to_dest_while_taking_opponent_piece_if_needed(
    board: &absolute::Board,
    src: absolute::Coord,
    dest: absolute::Coord,
    whose_turn: absolute::Side,
) -> Result<(absolute::Board, Option<absolute::NonTam2Piece>), &'static str> {
    let mut new_board = board.clone();

    let src_piece = new_board
        .remove(&src)
        .ok_or("src does not contain a piece")?;
    if src_piece.is_tam2() {
        return Err("Expected a NonTam2Piece to be present at the src, but found a Tam2");
    }

    if !src_piece.has_side(whose_turn) {
        return Err("Found the opponent piece at the src");
    }

    let maybe_captured_piece = new_board.remove(&dest);
    new_board.insert(dest, src_piece);

    if let Some(captured_piece) = maybe_captured_piece {
        match captured_piece {
            absolute::Piece::Tam2 => return Err("Tried to capture a Tam2"),
            absolute::Piece::NonTam2Piece {
                color: captured_piece_color,
                prof: captured_piece_prof,
                side: captured_piece_side,
            } => {
                if captured_piece_side == whose_turn {
                    return Err("Tried to capture an ally");
                }
                return Ok((
                    new_board,
                    Some(absolute::NonTam2Piece {
                        color: captured_piece_color,
                        prof: captured_piece_prof,
                    }),
                ));
            }
        }
    }
    return Ok((new_board, None));
}

pub fn apply_after_half_acceptance(
    old_state: &StateC,
    msg: AfterHalfAcceptance,
) -> Result<Probabilistic<ExistenceOfHandNotResolved>, &'static str> {
    let nothing_happened = ExistenceOfHandNotResolved {
        previous_a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
        previous_ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
        kut2tam2_happened: old_state.piece_at_flying_piece_step().is_tam2(),
        rate: old_state.rate,
        tam_has_moved_previously: false,
        season: old_state.season,
        tam_itself_is_tam_hue: old_state.tam_itself_is_tam_hue,
        ia_owner_s_score: old_state.ia_owner_s_score,
        whose_turn: old_state.whose_turn,
        f: old_state.f.clone(),
    };

    let StateC {
        flying_piece_src, ..
    } = *old_state;

    if let Some(dest) = msg.dest {
        let piece = old_state.piece_at_flying_piece_src();

        let (new_board, maybe_captured_piece) =
            move_nontam_piece_from_src_to_dest_while_taking_opponent_piece_if_needed(
                &old_state.f.board,
                flying_piece_src,
                dest,
                old_state.whose_turn,
            )?;

        let mut new_field = absolute::Field {
            board: new_board,
            ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
            a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
        };

        if let Some(absolute::NonTam2Piece { color, prof }) = maybe_captured_piece {
            new_field.insert_nontam_piece_into_hop1zuo1(color, prof, old_state.whose_turn);
        };

        // 入水判定不存在、または成功
        let success = ExistenceOfHandNotResolved {
            previous_a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
            previous_ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
            kut2tam2_happened: old_state.piece_at_flying_piece_step().is_tam2(),
            rate: old_state.rate,
            tam_has_moved_previously: false,
            season: old_state.season,
            tam_itself_is_tam_hue: old_state.tam_itself_is_tam_hue,
            ia_owner_s_score: old_state.ia_owner_s_score,
            whose_turn: old_state.whose_turn,
            f: new_field,
        };

        // Trying to enter the water without any exemptions (neither the piece started from within water, nor the piece is a Vessel).
        // Hence sticks must be cast.
        // 入水判定が免除される特例（出発地点が皇水であるか、移動している駒が船である）なしで水に入ろうとしているので、判定が必要。
        if !absolute::is_water(flying_piece_src)
            && !piece.has_prof(cetkaik_core::Profession::Nuak1)
            && absolute::is_water(dest)
        {
            return Ok(Probabilistic::Water {
                success,
                failure: nothing_happened,
            });
        } else {
            // 入水判定が絶対にないので確率は1
            // succeeds with probability 1
            return Ok(Probabilistic::Pure(success));
        }
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
