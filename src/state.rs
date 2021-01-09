use super::{absolute, state, IfTaxot, Rate, Scores, Season};
use serde::{Deserialize, Serialize};

/// Normal state. ／一番普通の状態。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct A {
    pub f: absolute::Field,
    pub whose_turn: absolute::Side,
    pub season: Season,
    pub scores: Scores,
    pub rate: Rate,
    pub tam_has_moved_previously: bool,
}

impl state::A {
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
        });

        let candidates = not_from_hop1zuo1_candidates_(
            &cetkaik_yhuap_move_candidates::Config {
                allow_kut2tam2: true,
            },
            &PureGameState {
                perspective,
                opponent_has_just_moved_tam: self.tam_has_moved_previously,
                tam_itself_is_tam_hue: config.tam_itself_is_tam_hue,
                f: to_relative_field(self.f.clone(), perspective),
            },
        );

        (
            hop1zuo1_candidates
                .into_iter()
                .map(super::message::PureMove::from)
                .collect(),
            candidates
                .into_iter()
                .map(super::message::PureMove::from)
                .collect(),
        )
    }
}

/// This is the state after the user has stepped over a piece and has cast the sticks so that the user can play to make an infinite movement from there. Seeing the sticks, the user is supposed to decide the final location and send it (`AfterHalfAcceptance`) to the server.
/// ／踏越え後の無限移動をユーザーが行い、それに対して投げ棒で判定した後の状態。投げ棒を見て、ユーザーは最終的な移動場所をCに対しこれから送りつける。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct C {
    pub c: CWithoutCiurl,
    pub ciurl: i32,
}

impl state::C {
    #[must_use]
    pub fn get_candidates(&self, config: super::Config) -> Vec<super::message::AfterHalfAcceptance> {
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
            ans.push(super::message::AfterHalfAcceptance { dest: Some(dest) })
        }
        ans
    }
}

/// Same as `C`, except that the ciurl is not mentioned.
/// ／`C` から投げ棒の値を除いたやつ。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CWithoutCiurl {
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
/// ／`HandNotResolved` を `resolve` でこの型に変換することによって、『役は発生しなかったぞ』であるのか、それとも『役は発生しており、したがって【再行ならこの `A` に至る】【終季ならこの `Probabilistic<state::A>` に至る（どちらが先手になるかは鯖のみぞ知るので `Probabilistic`）】』のどちらであるかを知ることができる。撃皇が役を構成するかどうかによってここの処理は変わってくるので、
/// `resolve` は `Config` を要求する。
#[derive(Clone, Debug)]
pub enum HandResolved {
    NeitherTymokNorTaxot(state::A),
    HandExists {
        if_tymok: state::A,
        if_taxot: IfTaxot,
    },

    /// 減点行為が役でないルールでは、役が成立して終季・再行の選択が発生せずに点が尽きることがありうる
    GameEndsWithoutTymokTaxot(super::score::Victor),
}
