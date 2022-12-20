use cetkaik_core::PureMove_;

use super::absolute;

pub type PureMove = PureMove__<absolute::Coord>;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PureMove__<T> {
    InfAfterStep(InfAfterStep_<T>),
    NormalMove(NormalMove_<T>),
}

impl<T> From<PureMove_<T>> for PureMove__<T> {
    fn from(candidate: PureMove_<T>) -> Self {
        match candidate {
            PureMove_::TamMoveNoStep {
                src,
                first_dest,
                second_dest,
            } => Self::NormalMove(NormalMove_::TamMoveNoStep {
                src,
                first_dest,
                second_dest,
            }),

            PureMove_::TamMoveStepsDuringFormer {
                src,
                step,
                first_dest,
                second_dest,
            } => Self::NormalMove(NormalMove_::TamMoveStepsDuringFormer {
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
            } => Self::NormalMove(NormalMove_::TamMoveStepsDuringLatter {
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
            } => Self::NormalMove(NormalMove_::NonTamMoveSrcStepDstFinite { src, step, dest }),

            PureMove_::InfAfterStep {
                src,
                step,
                planned_direction,
            } => Self::InfAfterStep(InfAfterStep_ {
                src,
                step,
                planned_direction,
            }),

            PureMove_::NonTamMoveFromHopZuo { color, prof, dest } => {
                Self::NormalMove(NormalMove_::NonTamMoveFromHopZuo { color, prof, dest })
            }

            PureMove_::NonTamMoveSrcDst {
                src,
                dest,
                is_water_entry_ciurl: _,
            } => Self::NormalMove(NormalMove_::NonTamMoveSrcDst { src, dest }),
        }
    }
}

pub type InfAfterStep = InfAfterStep_<absolute::Coord>;

/// Describes the moves that require a stepping-over cast
/// (that is, when after stepping over a piece you plan to make a movement with infinite range).
/// ／踏越え判定が必要になるタイプの移動を表現する型。
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct InfAfterStep_<T> {
    pub src: T,
    pub step: T,
    pub planned_direction: T,
}

pub type AfterHalfAcceptance = AfterHalfAcceptance_<absolute::Coord>;

/// Describes the decision after the stepping-over cast was sent from the server
/// ／踏越え判定の結果がサーバーから送られた後にユーザーが送ってくる決断を表現する型。
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct AfterHalfAcceptance_<T> {
    /// None: hands over the turn to the opponent
    /// None は（投げ棒の出目が気に入らなかったために）パスして相手に手番を渡すことを表す
    pub dest: Option<T>,
}

pub type NormalMove = NormalMove_<absolute::Coord>;

/// Describes all the moves except those that require a stepping-over cast
/// (that is, when after stepping over a piece you plan to make a movement with infinite range).
/// ／踏越え判定が不要なタイプの移動を表現する型。
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum NormalMove_<T> {
    NonTamMoveSrcDst {
        src: T,
        dest: T,
    },
    NonTamMoveSrcStepDstFinite {
        src: T,
        step: T,
        dest: T,
    },
    NonTamMoveFromHopZuo {
        color: cetkaik_core::Color,
        prof: cetkaik_core::Profession,
        dest: T,
    },
    TamMoveNoStep {
        src: T,
        first_dest: T,
        second_dest: T,
    },
    TamMoveStepsDuringFormer {
        src: T,
        step: T,
        first_dest: T,
        second_dest: T,
    },
    TamMoveStepsDuringLatter {
        src: T,
        step: T,
        first_dest: T,
        second_dest: T,
    },
}
