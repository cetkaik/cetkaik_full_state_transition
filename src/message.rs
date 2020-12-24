use super::*;

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
