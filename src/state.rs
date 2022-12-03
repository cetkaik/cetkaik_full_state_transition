use super::{absolute, state, IfTaxot, Rate, Scores, Season};
use serde::{Deserialize, Serialize};

/// Normal state. ／一番普通の状態。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroundState {
    pub f: absolute::Field,
    pub whose_turn: absolute::Side,
    pub season: Season,
    pub scores: Scores,
    pub rate: Rate,
    pub tam_has_moved_previously: bool,
}

impl state::GroundState {
    /// ```
    /// use cetkaik_full_state_transition::message::InfAfterStep;
    /// use cetkaik_full_state_transition::*;
    /// use cetkaik_core::absolute;
    /// use cetkaik_core::absolute::Coord;
    /// use cetkaik_core::absolute::Row::*;
    /// use cetkaik_core::absolute::Column::*;
    /// let ia_first = state::GroundState {
    ///     whose_turn: absolute::Side::IASide,
    ///     scores: Scores::new(),
    ///     rate: Rate::X1,
    ///     season: Season::Iei2,
    ///     tam_has_moved_previously: false,
    ///     f: absolute::Field {
    ///         a_side_hop1zuo1: vec![],
    ///         ia_side_hop1zuo1: vec![],
    ///         board: cetkaik_core::absolute::yhuap_initial_board(),
    ///     },
    /// };
    /// let (hop1zuo1_candidates, candidates) = ia_first.get_candidates(Config::cerke_online_alpha());
    /// assert_eq!(hop1zuo1_candidates, vec![]);
    /// let inf_after_step: Vec<_> = candidates.into_iter()
    ///     .filter_map(|a|
    ///         match a {
    ///             message::PureMove::InfAfterStep(m) => Some(m),
    ///             _ => None
    ///         }
    ///     ).collect();
    /// assert_eq!(inf_after_step, vec![
    ///     InfAfterStep { src: Coord(IA, P), step: Coord(AU, P), planned_direction: Coord(IA, P) },
    ///     InfAfterStep { src: Coord(IA, K), step: Coord(AU, K), planned_direction: Coord(IA, K) },
    ///     InfAfterStep { src: Coord(AU, P), step: Coord(AU, M), planned_direction: Coord(AU, C) },
    ///     InfAfterStep { src: Coord(AU, P), step: Coord(AU, M), planned_direction: Coord(AU, P) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(Y, M) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(O, M) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(U, M) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(I, M) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(AU, M) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(IA, M), planned_direction: Coord(AU, M) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(AU, X), planned_direction: Coord(AU, Z) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(AU, X), planned_direction: Coord(AU, C) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(AU, X), planned_direction: Coord(AU, M) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(AU, P), planned_direction: Coord(AU, M) },
    ///     InfAfterStep { src: Coord(AU, M), step: Coord(AU, P), planned_direction: Coord(AU, C) },
    ///     InfAfterStep { src: Coord(AU, X), step: Coord(AI, C), planned_direction: Coord(Y, X) },
    ///     InfAfterStep { src: Coord(AU, X), step: Coord(AI, C), planned_direction: Coord(O, P) },
    ///     InfAfterStep { src: Coord(AU, X), step: Coord(AI, C), planned_direction: Coord(Y, M) },
    ///     InfAfterStep { src: Coord(AU, X), step: Coord(AI, C), planned_direction: Coord(AU, X) },
    ///     InfAfterStep { src: Coord(AU, T), step: Coord(AI, N), planned_direction: Coord(O, K) },
    ///     InfAfterStep { src: Coord(AU, T), step: Coord(AI, N), planned_direction: Coord(Y, L) },
    ///     InfAfterStep { src: Coord(AU, T), step: Coord(AI, N), planned_direction: Coord(Y, T) },
    ///     InfAfterStep { src: Coord(AU, T), step: Coord(AI, N), planned_direction: Coord(AU, T) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(AI, L), planned_direction: Coord(Y, L) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(AI, L), planned_direction: Coord(O, L) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(AI, L), planned_direction: Coord(U, L) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(AI, L), planned_direction: Coord(I, L) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(AI, L), planned_direction: Coord(AU, L) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(IA, L), planned_direction: Coord(AU, L) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(AU, K), planned_direction: Coord(AU, L) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(AU, K), planned_direction: Coord(AU, N) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(AU, T), planned_direction: Coord(AU, N) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(AU, T), planned_direction: Coord(AU, L) },
    ///     InfAfterStep { src: Coord(AU, L), step: Coord(AU, T), planned_direction: Coord(AU, Z) },
    ///     InfAfterStep { src: Coord(AU, K), step: Coord(AU, L), planned_direction: Coord(AU, K) },
    ///     InfAfterStep { src: Coord(AU, K), step: Coord(AU, L), planned_direction: Coord(AU, N) },
    ///     InfAfterStep { src: Coord(AI, Z), step: Coord(O, Z), planned_direction: Coord(U, Z) },
    ///     InfAfterStep { src: Coord(AI, Z), step: Coord(O, Z), planned_direction: Coord(I, Z) },
    ///     InfAfterStep { src: Coord(AI, Z), step: Coord(O, Z), planned_direction: Coord(Y, Z) },
    ///     InfAfterStep { src: Coord(AI, Z), step: Coord(O, Z), planned_direction: Coord(AI, Z) },
    ///     InfAfterStep { src: Coord(AI, Z), step: Coord(O, Z), planned_direction: Coord(AU, Z) }
    /// ])
    /// ```
    #[must_use]
    pub fn get_candidates(
        &self,
        config: super::Config,
    ) -> (Vec<super::message::PureMove>, Vec<super::message::PureMove>) {
        use cetkaik_yhuap_move_candidates::{
            from_hop1zuo1_candidates, not_from_hop1zuo1_candidates_, to_relative_field,
            PureGameState,
        };

        // must set it so that self.whose_turn points downward
        let perspective = match self.whose_turn {
            absolute::Side::IASide => {
                cetkaik_core::perspective::Perspective::IaIsUpAndPointsDownward
            }
            absolute::Side::ASide => {
                cetkaik_core::perspective::Perspective::IaIsDownAndPointsUpward
            }
        };

        let hop1zuo1_candidates = from_hop1zuo1_candidates(&PureGameState {
            perspective,
            opponent_has_just_moved_tam: self.tam_has_moved_previously,
            tam_itself_is_tam_hue: config.tam_itself_is_tam_hue,
            f: to_relative_field(self.f.clone(), perspective),
        })
        .into_iter()
        .map(super::message::PureMove::from)
        .collect::<Vec<_>>();

        let mut candidates = not_from_hop1zuo1_candidates_(
            &cetkaik_yhuap_move_candidates::Config {
                allow_kut2tam2: true,
            },
            &PureGameState {
                perspective,
                opponent_has_just_moved_tam: self.tam_has_moved_previously,
                tam_itself_is_tam_hue: config.tam_itself_is_tam_hue,
                f: to_relative_field(self.f.clone(), perspective),
            },
        )
        .into_iter()
        .map(super::message::PureMove::from)
        .collect::<Vec<_>>();

        if self.tam_has_moved_previously
            && config.moving_tam_immediately_after_tam_has_moved == super::Consequence::Forbidden
        {
            candidates.retain(|a| {
                    !matches!(
                        a,
                        super::message::PureMove::NormalMove(
                            super::message::NormalMove::TamMoveNoStep { .. }
                                | super::message::NormalMove::TamMoveStepsDuringFormer { .. }
                                | super::message::NormalMove::TamMoveStepsDuringLatter { .. },
                        )
                    )
                });
        }

        if config.tam_mun_mok == super::Consequence::Forbidden {
            candidates.retain(|a| {
                    match a {
                        super::message::PureMove::NormalMove(
                            super::message::NormalMove::TamMoveNoStep {
                                src, second_dest, ..
                            }
                            | super::message::NormalMove::TamMoveStepsDuringFormer {
                                src,
                                second_dest,
                                ..
                            }
                            | super::message::NormalMove::TamMoveStepsDuringLatter {
                                src,
                                second_dest,
                                ..
                            },
                        ) => src != second_dest, /* false when mun1mok1 */
                        _ => true, /* always allow */
                    }
                });
        }

        (hop1zuo1_candidates, candidates)
    }
}

/// This is the state after the user has stepped over a piece and has cast the sticks so that the user can play to make an infinite movement from there. Seeing the sticks, the user is supposed to decide the final location and send it (`AfterHalfAcceptance`) to the server.
/// ／踏越え後の無限移動をユーザーが行い、それに対して投げ棒で判定した後の状態。投げ棒を見て、ユーザーは最終的な移動場所を `ExcitedState` に対しこれから送りつける。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExcitedState {
    pub c: ExcitedStateWithoutCiurl,
    pub ciurl: i32,
}

impl state::ExcitedState {
    #[must_use]
    pub fn get_candidates(
        &self,
        config: super::Config,
    ) -> Vec<super::message::AfterHalfAcceptance> {
        let perspective = match self.c.whose_turn {
            absolute::Side::IASide => {
                cetkaik_core::perspective::Perspective::IaIsUpAndPointsDownward
            }
            absolute::Side::ASide => {
                cetkaik_core::perspective::Perspective::IaIsDownAndPointsUpward
            }
        };
        let candidates = cetkaik_yhuap_move_candidates::not_from_hop1zuo1_candidates_(
            &cetkaik_yhuap_move_candidates::Config {
                allow_kut2tam2: true,
            },
            &cetkaik_yhuap_move_candidates::PureGameState {
                perspective,
                opponent_has_just_moved_tam: false, /* it doesn't matter */
                tam_itself_is_tam_hue: config.tam_itself_is_tam_hue,
                f: cetkaik_yhuap_move_candidates::to_relative_field(self.c.f.clone(), perspective),
            },
        );

        let destinations = candidates.into_iter().filter_map(|cand| match cand {
            cetkaik_yhuap_move_candidates::PureMove::InfAfterStep {
                src,
                step,
                planned_direction,
            } => {
                if src == self.c.flying_piece_src
                    && step == self.c.flying_piece_step
                    && self.ciurl >= cetkaik_core::absolute::distance(step, planned_direction)
                /*
                must also check whether the ciurl limit is not violated
                投げ棒による距離限界についても検査が要る
                */
                {
                    Some(planned_direction)
                } else {
                    None
                }
            }
            _ => None,
        });

        let mut ans = vec![super::message::AfterHalfAcceptance { dest: None }];

        for dest in destinations {
            ans.push(super::message::AfterHalfAcceptance { dest: Some(dest) });
        }
        ans
    }
}

/// Same as `ExcitedState`, except that the ciurl is not mentioned.
/// ／`ExcitedState` から投げ棒の値を除いたやつ。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExcitedStateWithoutCiurl {
    pub f: absolute::Field,
    pub whose_turn: absolute::Side,
    pub flying_piece_src: absolute::Coord,
    pub flying_piece_step: absolute::Coord,
    pub season: Season,
    pub scores: Scores,
    pub rate: Rate,
}

/// The water entry cast (if any) is now over, and thus the piece movement is now fully completed. However, I still haven't resolved whether a hand exists. If so, I must ask the user to choose whether to end the season or not.
/// ／入水判定も終わり、駒を完全に動かし終わった。しかしながら、「役が存在していて再行・終季をユーザーに訊く」を発生させるか否かをまだ解決していない。そんな状態。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HandNotResolved {
    pub f: absolute::Field,
    pub whose_turn: absolute::Side,
    pub season: Season,
    pub scores: Scores,
    pub rate: Rate,
    pub i_have_moved_tam_in_this_turn: bool,
    pub previous_a_side_hop1zuo1: Vec<absolute::NonTam2Piece>,
    pub previous_ia_side_hop1zuo1: Vec<absolute::NonTam2Piece>,
    pub kut2tam2_happened: bool,
    pub tam2tysak2_raw_penalty: i32,

    /// Even when this field is set, the penalty is already subtracted from `ia_owner_s_score`
    /// ／このフィールドが `true` であるときも、罰則点はすでに `ia_owner_s_score` に計上してあるので、調整しなくてよい。
    pub tam2tysak2_will_trigger_taxottymok: bool,
}

/// Converting `HandNotResolved` into `HandResolved` with `resolve` tells you whether a new hand was created. If so, the `HandExists` variant is taken; if not, the `NeitherTymokNorTaxot` is taken.
/// ／`HandNotResolved` を `resolve` でこの型に変換することによって、『役は発生しなかったぞ』であるのか、それとも『役は発生しており、したがって【再行ならこの `GroundState` に至る】【終季ならこの `Probabilistic<state::GroundState>` に至る（どちらが先手になるかは鯖のみぞ知るので `Probabilistic`）】』のどちらであるかを知ることができる。撃皇が役を構成するかどうかによってここの処理は変わってくるので、
/// `resolve` は `Config` を要求する。
#[derive(Clone, Debug)]
pub enum HandResolved {
    NeitherTymokNorTaxot(state::GroundState),
    HandExists {
        if_tymok: state::GroundState,
        if_taxot: IfTaxot,
    },

    /// 減点行為が役でないルールでは、役が成立して終季・再行の選択が発生せずに点が尽きることがありうる
    GameEndsWithoutTymokTaxot(super::score::Victor),
}
