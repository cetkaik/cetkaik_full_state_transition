use cetkaik_core::PureMove_;

use super::absolute;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PureMove {
    InfAfterStep(InfAfterStep),
    NormalMove(NormalMove),
}

impl From<PureMove_<absolute::Coord>> for PureMove {
    fn from(candidate: PureMove_<absolute::Coord>) -> Self {
        match candidate {
            PureMove_::TamMoveNoStep {
                src,
                first_dest,
                second_dest,
            } => Self::NormalMove(NormalMove::TamMoveNoStep {
                src,
                first_dest,
                second_dest,
            }),

            PureMove_::TamMoveStepsDuringFormer {
                src,
                step,
                first_dest,
                second_dest,
            } => Self::NormalMove(NormalMove::TamMoveStepsDuringFormer {
                src,
                step,
                first_dest,
                second_dest,
            }),

            PureMove_::TamMoveStepsDuringLatter {
                src,
                step,
                first_dest,
                second_dest,
            } => Self::NormalMove(NormalMove::TamMoveStepsDuringLatter {
                src,
                step,
                first_dest,
                second_dest,
            }),

            PureMove_::NonTamMoveSrcStepDstFinite {
                src,
                step,
                dest,
                is_water_entry_ciurl: _,
            } => Self::NormalMove(NormalMove::NonTamMoveSrcStepDstFinite { src, step, dest }),

            PureMove_::InfAfterStep {
                src,
                step,
                planned_direction,
            } => Self::InfAfterStep(InfAfterStep {
                src,
                step,
                planned_direction,
            }),

            PureMove_::NonTamMoveFromHopZuo { color, prof, dest } => {
                Self::NormalMove(NormalMove::NonTamMoveFromHopZuo { color, prof, dest })
            }

            PureMove_::NonTamMoveSrcDst {
                src,
                dest,
                is_water_entry_ciurl: _,
            } => Self::NormalMove(NormalMove::NonTamMoveSrcDst { src, dest }),
        }
    }
}

/// Describes the moves that require a stepping-over cast
/// (that is, when after stepping over a piece you plan to make a movement with infinite range).
/// ／踏越え判定が必要になるタイプの移動を表現する型。
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct InfAfterStep {
    pub src: absolute::Coord,
    pub step: absolute::Coord,
    pub planned_direction: absolute::Coord,
}

/// Describes the decision after the stepping-over cast was sent from the server
/// ／踏越え判定の結果がサーバーから送られた後にユーザーが送ってくる決断を表現する型。
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct AfterHalfAcceptance {
    /// None: hands over the turn to the opponent
    /// None は（投げ棒の出目が気に入らなかったために）パスして相手に手番を渡すことを表す
    pub dest: Option<absolute::Coord>,
}

/// Describes all the moves except those that require a stepping-over cast
/// (that is, when after stepping over a piece you plan to make a movement with infinite range).
/// ／踏越え判定が不要なタイプの移動を表現する型。
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum NormalMove {
    NonTamMoveSrcDst {
        src: absolute::Coord,
        dest: absolute::Coord,
    },
    NonTamMoveSrcStepDstFinite {
        src: absolute::Coord,
        step: absolute::Coord,
        dest: absolute::Coord,
    },
    NonTamMoveFromHopZuo {
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
