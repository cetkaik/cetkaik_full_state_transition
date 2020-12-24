#[warn(clippy::pedantic)]
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

pub mod message;
pub mod probabilistic;

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

pub mod state;

impl state::C {
    pub fn piece_at_flying_piece_src(&self) -> absolute::Piece {
        *self
            .c
            .f
            .board
            .get(&self.c.flying_piece_src)
            .expect("Invalid state::C: at flying_piece_src there is no piece")
    }

    pub fn piece_at_flying_piece_step(&self) -> absolute::Piece {
        *self
            .c
            .f
            .board
            .get(&self.c.flying_piece_step)
            .expect("Invalid state::C: at flying_piece_step there is no piece")
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
    pub fn next(self) -> Self {
        match self {
            Rate::X1 => Rate::X2,
            Rate::X2 => Rate::X4,
            Rate::X4 => Rate::X8,
            Rate::X8 => Rate::X16,
            Rate::X16 => Rate::X32,
            Rate::X32 => Rate::X64,
            Rate::X64 => Rate::X64,
        }
    }
    pub fn num(self) -> i32 {
        match self {
            Rate::X1 => 1,
            Rate::X2 => 2,
            Rate::X4 => 4,
            Rate::X8 => 8,
            Rate::X16 => 16,
            Rate::X32 => 32,
            Rate::X64 => 64,
        }
    }
}

fn apply_tam_move(
    old_state: &state::A,
    src: absolute::Coord,
    first_dest: absolute::Coord,
    second_dest: absolute::Coord,
    step: Option<absolute::Coord>,
) -> Result<Probabilistic<state::HandNotResolved>, &'static str> {
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
    return Ok(Probabilistic::Pure(state::HandNotResolved {
        previous_a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
        previous_ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
        kut2tam2_happened: false,
        rate: old_state.rate,
        tam_has_moved_previously: true,
        season: old_state.season,
        ia_owner_s_score: old_state.ia_owner_s_score,
        whose_turn: old_state.whose_turn,
        f: new_field,
    }));
}

fn apply_nontam_move(
    old_state: &state::A,
    src: absolute::Coord,
    dest: absolute::Coord,
    step: Option<absolute::Coord>,
) -> Result<Probabilistic<state::HandNotResolved>, &'static str> {
    let nothing_happened = state::HandNotResolved {
        previous_a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
        previous_ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
        kut2tam2_happened: match step {
            None => false,
            Some(step) => match old_state.f.board.get(&step) {
                Some(absolute::Piece::Tam2) => true,
                _ => false,
            },
        },
        rate: old_state.rate,
        tam_has_moved_previously: false,
        season: old_state.season,
        ia_owner_s_score: old_state.ia_owner_s_score,
        whose_turn: old_state.whose_turn,
        f: old_state.f.clone(),
    };

    if let Some(st) = step {
        if !old_state.f.board.contains_key(&st) {
            return Err("expected a stepping square but found an empty square");
        }
    }

    let src_piece = old_state
        .f
        .board
        .get(&src)
        .ok_or("src does not contain a piece")?;

    let (new_board, maybe_captured_piece) =
        move_nontam_piece_from_src_to_dest_while_taking_opponent_piece_if_needed(
            &old_state.f.board,
            src,
            dest,
            old_state.whose_turn,
        )?;
    let mut new_field = absolute::Field {
        board: new_board,
        a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
        ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
    };

    if let Some(absolute::NonTam2Piece { color, prof }) = maybe_captured_piece {
        new_field.insert_nontam_piece_into_hop1zuo1(color, prof, old_state.whose_turn);
    }

    let success = state::HandNotResolved {
        previous_a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
        previous_ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
        kut2tam2_happened: false,
        rate: old_state.rate,
        tam_has_moved_previously: false,
        season: old_state.season,
        ia_owner_s_score: old_state.ia_owner_s_score,
        whose_turn: old_state.whose_turn,
        f: new_field,
    };

    // 入水判定
    // water-entry cast
    if !absolute::is_water(src)
        && !src_piece.has_prof(cetkaik_core::Profession::Nuak1)
        && absolute::is_water(dest)
    {
        return Ok(Probabilistic::Water {
            failure: nothing_happened,
            success,
        });
    }
    return Ok(Probabilistic::Pure(success));
}

pub fn apply_normal_move(
    old_state: &state::A,
    msg: message::NormalMove,
    config: Config,
) -> Result<Probabilistic<state::HandNotResolved>, &'static str> {
    use cetkaik_yhuap_move_candidates::*;

    // must set it so that old_state.whose_turn points downward
    let perspective = match old_state.whose_turn {
        absolute::Side::IASide => cetkaik_core::perspective::Perspective::IaIsUpAndPointsDownward,
        absolute::Side::ASide => cetkaik_core::perspective::Perspective::IaIsDownAndPointsUpward,
    };

    let hop1zuo1_candidates = from_hand_candidates(&PureGameState {
        perspective,
        opponent_has_just_moved_tam: old_state.tam_has_moved_previously,
        tam_itself_is_tam_hue: config.tam_itself_is_tam_hue,
        f: to_relative_field(old_state.f.clone(), perspective),
    });

    let candidates = not_from_hand_candidates_(
        &cetkaik_yhuap_move_candidates::Config {
            allow_kut2tam2: true,
        },
        &PureGameState {
            perspective,
            opponent_has_just_moved_tam: old_state.tam_has_moved_previously,
            tam_itself_is_tam_hue: config.tam_itself_is_tam_hue,
            f: to_relative_field(old_state.f.clone(), perspective),
        },
    );

    match msg {
        message::NormalMove::NonTamMoveFromHand { color, prof, dest } => {
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

            // For the sake of consistency, `cetkaik_yhuap_move_candidates` is called,
            // but since all illegal moves from hop1zuo1 are those that are trivially illegal
            // (that is, "you can't place onto a non-empty square" and "you can't place what you don't have"),
            // `cetkaik_yhuap_move_candidates` should never report a failure.
            // that is why it is not `return Err` but is `unreachable`.
            // 念のため `cetkaik_yhuap_move_candidates` を呼び出しておくが、
            // 持ち駒から打つ際の違法手というのが実は自明なものを除いて不存在なので、
            // ここで弾かれるとしたらコードがバグっているということになる。
            // したがって、 return Err ではなく unreachable としてある。
            {
                if !hop1zuo1_candidates.contains(
                    &cetkaik_yhuap_move_candidates::PureMove::NonTamMoveFromHand {
                        color,
                        prof,
                        dest,
                    },
                ) {
                    unreachable!("inconsistencies found between cetkaik_yhuap_move_candidates::PureMove::NonTamMoveFromHand and cetkaik_full_state_transition::apply_nontam_move")
                }
            }

            return Ok(Probabilistic::Pure(state::HandNotResolved {
                previous_a_side_hop1zuo1: old_state.f.a_side_hop1zuo1.clone(),
                previous_ia_side_hop1zuo1: old_state.f.ia_side_hop1zuo1.clone(),
                kut2tam2_happened: false,
                rate: old_state.rate,
                tam_has_moved_previously: false,
                season: old_state.season,
                ia_owner_s_score: old_state.ia_owner_s_score,
                whose_turn: old_state.whose_turn,
                f: new_field,
            }));
        }
        message::NormalMove::TamMoveNoStep {
            src,
            first_dest,
            second_dest,
        } => {
            if candidates.contains(&cetkaik_yhuap_move_candidates::PureMove::TamMoveNoStep {
                src,
                first_dest,
                second_dest,
            }) {
                return apply_tam_move(old_state, src, first_dest, second_dest, None);
            } else {
                return Err("The provided TamMoveNoStep was rejected by the crate `cetkaik_yhuap_move_candidates`.");
            }
        }
        message::NormalMove::TamMoveStepsDuringFormer {
            src,
            first_dest,
            second_dest,
            step,
        } => {
            if candidates.contains(
                &cetkaik_yhuap_move_candidates::PureMove::TamMoveStepsDuringFormer {
                    src,
                    first_dest,
                    second_dest,
                    step,
                },
            ) {
                return apply_tam_move(old_state, src, first_dest, second_dest, Some(step));
            } else {
                return Err("The provided TamMoveStepsDuringFormer was rejected by the crate `cetkaik_yhuap_move_candidates`.");
            }
        }
        message::NormalMove::TamMoveStepsDuringLatter {
            src,
            first_dest,
            second_dest,
            step,
        } => {
            if candidates.contains(
                &cetkaik_yhuap_move_candidates::PureMove::TamMoveStepsDuringLatter {
                    src,
                    first_dest,
                    second_dest,
                    step,
                },
            ) {
                return apply_tam_move(old_state, src, first_dest, second_dest, Some(step));
            } else {
                return Err("The provided TamMoveStepsDuringLatter was rejected by the crate `cetkaik_yhuap_move_candidates`.");
            }
        }

        message::NormalMove::NonTamMoveSrcDst { src, dest } => {
            if candidates.contains(&cetkaik_yhuap_move_candidates::PureMove::NonTamMoveSrcDst {
                src,
                dest,
                is_water_entry_ciurl: true,
            }) || candidates.contains(
                &cetkaik_yhuap_move_candidates::PureMove::NonTamMoveSrcDst {
                    src,
                    dest,
                    is_water_entry_ciurl: false,
                },
            ) {
                return apply_nontam_move(old_state, src, dest, None);
            } else {
                return Err("The provided NonTamMoveSrcDst was rejected by the crate `cetkaik_yhuap_move_candidates`.");
            }
        }
        message::NormalMove::NonTamMoveSrcStepDstFinite { src, step, dest } => {
            if candidates.contains(
                &cetkaik_yhuap_move_candidates::PureMove::NonTamMoveSrcStepDstFinite {
                    src,
                    step,
                    dest,
                    is_water_entry_ciurl: true,
                },
            ) || candidates.contains(
                &cetkaik_yhuap_move_candidates::PureMove::NonTamMoveSrcStepDstFinite {
                    src,
                    step,
                    dest,
                    is_water_entry_ciurl: false,
                },
            ) {
                return apply_nontam_move(old_state, src, dest, Some(step));
            } else {
                return Err("The provided NonTamMoveSrcStepDstFinite was rejected by the crate `cetkaik_yhuap_move_candidates`.");
            }
        }
    }
}

pub fn apply_inf_after_step(
    old_state: &state::A,
    msg: message::InfAfterStep,
    config: Config,
) -> Result<Probabilistic<state::C>, &'static str> {
    if !old_state.f.board.contains_key(&msg.src) {
        return Err("In InfAfterStep, `src` is not occupied; illegal");
    }

    if !old_state.f.board.contains_key(&msg.step) {
        return Err("In InfAfterStep, `step` is not occupied; illegal");
    }

    let perspective = match old_state.whose_turn {
        absolute::Side::IASide => cetkaik_core::perspective::Perspective::IaIsUpAndPointsDownward,
        absolute::Side::ASide => cetkaik_core::perspective::Perspective::IaIsDownAndPointsUpward,
    };

    let candidates = cetkaik_yhuap_move_candidates::not_from_hand_candidates_(
        &cetkaik_yhuap_move_candidates::Config {
            allow_kut2tam2: true,
        },
        &cetkaik_yhuap_move_candidates::PureGameState {
            perspective,
            opponent_has_just_moved_tam: old_state.tam_has_moved_previously,
            tam_itself_is_tam_hue: config.tam_itself_is_tam_hue,
            f: cetkaik_yhuap_move_candidates::to_relative_field(old_state.f.clone(), perspective),
        },
    );

    if !candidates
        .into_iter()
        .filter(|cand| match cand {
            cetkaik_yhuap_move_candidates::PureMove::InfAfterStep {
                src,
                step,
                planned_direction: _,
            } => *src == msg.src && *step == msg.step,
            _ => false,
        })
        .count()
        > 0
    {
        return Err(
            "The provided InfAfterStep was rejected by the crate `cetkaik_yhuap_move_candidates`.",
        );
    }

    let c = state::CWithoutCiurl {
        f: old_state.f.clone(),
        whose_turn: old_state.whose_turn,
        flying_piece_src: msg.src,
        flying_piece_step: msg.step,
        season: old_state.season,
        ia_owner_s_score: old_state.ia_owner_s_score,
        rate: old_state.rate,
    };

    Ok(Probabilistic::Sticks {
        s0: state::C {
            c: c.clone(),
            ciurl: 0,
        },
        s1: state::C {
            c: c.clone(),
            ciurl: 1,
        },
        s2: state::C {
            c: c.clone(),
            ciurl: 2,
        },
        s3: state::C {
            c: c.clone(),
            ciurl: 3,
        },
        s4: state::C {
            c: c.clone(),
            ciurl: 4,
        },
        s5: state::C {
            c: c.clone(),
            ciurl: 5,
        },
    })
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
    old_state: &state::C,
    msg: message::AfterHalfAcceptance,
    config: Config,
) -> Result<Probabilistic<state::HandNotResolved>, &'static str> {
    let nothing_happened = state::HandNotResolved {
        previous_a_side_hop1zuo1: old_state.c.f.a_side_hop1zuo1.clone(),
        previous_ia_side_hop1zuo1: old_state.c.f.ia_side_hop1zuo1.clone(),
        kut2tam2_happened: old_state.piece_at_flying_piece_step().is_tam2(),
        rate: old_state.c.rate,
        tam_has_moved_previously: false,
        season: old_state.c.season,
        ia_owner_s_score: old_state.c.ia_owner_s_score,
        whose_turn: old_state.c.whose_turn,
        f: old_state.c.f.clone(),
    };

    let state::C {
        c: state::CWithoutCiurl {
            flying_piece_src, ..
        },
        ciurl
    } = *old_state;

    if let Some(dest) = msg.dest {
        // it is possible that the destination is illegal.
        // dest に変な値を突っ込まれることに対する対策が必要なので検閲する。
        {
            let perspective = match old_state.c.whose_turn {
                absolute::Side::IASide => {
                    cetkaik_core::perspective::Perspective::IaIsUpAndPointsDownward
                }
                absolute::Side::ASide => {
                    cetkaik_core::perspective::Perspective::IaIsDownAndPointsUpward
                }
            };
            let candidates = cetkaik_yhuap_move_candidates::not_from_hand_candidates_(
                &cetkaik_yhuap_move_candidates::Config {
                    allow_kut2tam2: true,
                },
                &cetkaik_yhuap_move_candidates::PureGameState {
                    perspective,
                    opponent_has_just_moved_tam: false, /* it doesn't matter */
                    tam_itself_is_tam_hue: config.tam_itself_is_tam_hue,
                    f: cetkaik_yhuap_move_candidates::to_relative_field(
                        old_state.c.f.clone(),
                        perspective,
                    ),
                },
            );
            if !candidates
                .into_iter()
                .filter(|cand| match cand {
                    cetkaik_yhuap_move_candidates::PureMove::InfAfterStep {
                        src,
                        step,
                        planned_direction,
                    } => {
                        *src == old_state.c.flying_piece_src
                            && *step == old_state.c.flying_piece_step
                            && *planned_direction == dest
                    }
                    _ => false,
                })
                .count()
                > 0
            {
                return Err(
                    "The provided InfAfterStep was rejected by the crate `cetkaik_yhuap_move_candidates`.",
                );
            }

            // must also check whether the ciurl limit is not violated
            // 投げ棒による距離限界についても検査が要る
            if ciurl < cetkaik_core::absolute::distance(old_state.c.flying_piece_step, dest) {
                return Err(
                    "The provided InfAfterStep was rejected because the ciurl limit was exceeded.",
                );
            }
        }

        let piece = old_state.piece_at_flying_piece_src();

        let (new_board, maybe_captured_piece) =
            move_nontam_piece_from_src_to_dest_while_taking_opponent_piece_if_needed(
                &old_state.c.f.board,
                flying_piece_src,
                dest,
                old_state.c.whose_turn,
            )?;

        let mut new_field = absolute::Field {
            board: new_board,
            ia_side_hop1zuo1: old_state.c.f.ia_side_hop1zuo1.clone(),
            a_side_hop1zuo1: old_state.c.f.a_side_hop1zuo1.clone(),
        };

        if let Some(absolute::NonTam2Piece { color, prof }) = maybe_captured_piece {
            new_field.insert_nontam_piece_into_hop1zuo1(color, prof, old_state.c.whose_turn);
        };

        // 入水判定不存在、または成功
        let success = state::HandNotResolved {
            previous_a_side_hop1zuo1: old_state.c.f.a_side_hop1zuo1.clone(),
            previous_ia_side_hop1zuo1: old_state.c.f.ia_side_hop1zuo1.clone(),
            kut2tam2_happened: old_state.piece_at_flying_piece_step().is_tam2(),
            rate: old_state.c.rate,
            tam_has_moved_previously: false,
            season: old_state.c.season,
            ia_owner_s_score: old_state.c.ia_owner_s_score,
            whose_turn: old_state.c.whose_turn,
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
        // modify the score (this side effect is to be handled by `resolve`). Water entry cannot fail,
        // since the piece has not actually moved.
        // 唯一ありえる副作用は、撃皇で点が減っている可能性があるということ（それは `resolve` で処理される）。
        // パスが発生した以上、駒の動きは実際には発生していないので、
        // 入水判定は発生していない。

        return Ok(Probabilistic::Pure(nothing_happened));
    }
}

pub enum IfTaxot {
    NextTurn(Probabilistic<state::A>),

    /// if VictoriousSide(Some(SideA)), SideA has won; if VictoriousSide(Some(SideIA)), SideIA has won.
    /// if VictoriousSide(None), the game is a draw.
    /// VictoriousSide(Some(SideA)) なら SideA が勝っており、 VictoriousSide(Some(SideIA)) なら SideIA が勝っている。
    /// VictoriousSide(None) なら引き分けである。
    VictoriousSide(Option<absolute::Side>),
}

pub struct Config {
    pub step_tam_is_a_hand: bool,
    pub tam_itself_is_tam_hue: bool,
}

pub fn resolve(state: state::HandNotResolved, config: Config) -> state::HandResolved {
    use cetkaik_calculate_hand::{calculate_hands_and_score_from_pieces, ScoreAndHands};
    let tymoxtaxot_because_of_kut2tam2 = state.kut2tam2_happened && config.step_tam_is_a_hand;

    let tymoxtaxot_because_of_newly_acquired: Option<i32> = match state.whose_turn {
        absolute::Side::ASide => {
            if state.previous_a_side_hop1zuo1 == state.f.a_side_hop1zuo1 {
                None
            } else {
                let ScoreAndHands {
                    score: _,
                    hands: old_hands,
                } = calculate_hands_and_score_from_pieces(&state.previous_a_side_hop1zuo1).unwrap();
                let ScoreAndHands {
                    score: new_score,
                    hands: new_hands,
                } = calculate_hands_and_score_from_pieces(&state.f.a_side_hop1zuo1).unwrap();

                // whether newly-acquired hand exists
                if new_hands.difference(&old_hands).count() > 0 {
                    Some(new_score)
                } else {
                    None
                }
            }
        }
        absolute::Side::IASide => {
            if state.previous_ia_side_hop1zuo1 == state.f.ia_side_hop1zuo1 {
                None
            } else {
                let ScoreAndHands {
                    score: _,
                    hands: old_hands,
                } = calculate_hands_and_score_from_pieces(&state.previous_ia_side_hop1zuo1)
                    .unwrap();
                let ScoreAndHands {
                    score: new_score,
                    hands: new_hands,
                } = calculate_hands_and_score_from_pieces(&state.f.ia_side_hop1zuo1).unwrap();

                // whether newly-acquired hand exists
                if new_hands.difference(&old_hands).count() > 0 {
                    Some(new_score)
                } else {
                    None
                }
            }
        }
    };

    let raw_score = match (
        tymoxtaxot_because_of_kut2tam2,
        tymoxtaxot_because_of_newly_acquired,
    ) {
        // nothing happened; hand the turn to the next person
        // 役ができていないので、次の人に手番を渡す
        // この際、step_tam_is_a_handがfalseの場合、5点×レートを引くだけ引く。
        (false, None) => {
            return state::HandResolved::NeitherTymokNorTaxot(state::A {
                f: state.f.clone(),
                whose_turn: !state.whose_turn, /* hand the turn to the next person */
                season: state.season,
                ia_owner_s_score: state.ia_owner_s_score
                    + if state.kut2tam2_happened {
                        state.rate.num()
                    } else {
                        0
                    } * match state.whose_turn {
                        absolute::Side::IASide => -5,
                        absolute::Side::ASide => 5,
                    },
                rate: state.rate,
                tam_has_moved_previously: state.tam_has_moved_previously,
            });
        }

        (false, Some(score)) => score,
        (true, None) => -5,
        (true, Some(score)) => score - 5,
    };

    let increment_in_ia_owner_s_score = match state.whose_turn {
        absolute::Side::IASide => 1,
        absolute::Side::ASide => -1,
    } * state.rate.num()
        * raw_score;

    let new_ia_owner_s_score =
        0.min(40.max(state.ia_owner_s_score + increment_in_ia_owner_s_score));
    let if_taxot = if new_ia_owner_s_score == 40 {
        IfTaxot::VictoriousSide(Some(absolute::Side::IASide))
    } else if new_ia_owner_s_score == 0 {
        IfTaxot::VictoriousSide(Some(absolute::Side::ASide))
    } else if let Some(next_season) = state.season.next() {
        let ia_first = state::A {
            whose_turn: absolute::Side::IASide,
            ia_owner_s_score: new_ia_owner_s_score,
            rate: Rate::X1,
            season: next_season,
            tam_has_moved_previously: state.tam_has_moved_previously,
            f: absolute::Field {
                a_side_hop1zuo1: vec![],
                ia_side_hop1zuo1: vec![],
                board: cetkaik_core::absolute::yhuap_initial_board(),
            },
        };
        let mut a_first = ia_first.clone();
        a_first.whose_turn = absolute::Side::ASide;
        IfTaxot::NextTurn(Probabilistic::WhoGoesFirst { ia_first, a_first })
    } else {
        /* All seasons have ended */
        use std::cmp::Ordering;
        match new_ia_owner_s_score.cmp(&(40 - new_ia_owner_s_score)) {
            Ordering::Greater => IfTaxot::VictoriousSide(Some(absolute::Side::IASide)),
            Ordering::Less => IfTaxot::VictoriousSide(Some(absolute::Side::ASide)),
            Ordering::Equal => IfTaxot::VictoriousSide(None),
        }
    };

    state::HandResolved::HandExists {
        if_tymok: state::A {
            f: state.f.clone(),
            whose_turn: !state.whose_turn, /* hand the turn to the next person */
            season: state.season,
            ia_owner_s_score: state.ia_owner_s_score,
            rate: state.rate.next(), /* double the stake */
            tam_has_moved_previously: state.tam_has_moved_previously,
        },

        if_taxot,
    }
}
