use super::absolute;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PureMove {
    InfAfterStep(InfAfterStep),
    NormalMove(NormalMove),
}

impl From<cetkaik_yhuap_move_candidates::PureMove> for PureMove {
    fn from(candidate: cetkaik_yhuap_move_candidates::PureMove) -> Self {
        match candidate {
            cetkaik_yhuap_move_candidates::PureMove::TamMoveNoStep {
                src,
                first_dest,
                second_dest,
            } => Self::NormalMove(NormalMove::TamMoveNoStep {
                src,
                first_dest,
                second_dest,
            }),

            cetkaik_yhuap_move_candidates::PureMove::TamMoveStepsDuringFormer {
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

            cetkaik_yhuap_move_candidates::PureMove::TamMoveStepsDuringLatter {
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

            cetkaik_yhuap_move_candidates::PureMove::NonTamMoveSrcStepDstFinite {
                src,
                step,
                dest,
                is_water_entry_ciurl: _,
            } => Self::NormalMove(NormalMove::NonTamMoveSrcStepDstFinite { src, step, dest }),

            cetkaik_yhuap_move_candidates::PureMove::InfAfterStep {
                src,
                step,
                planned_direction,
            } => Self::InfAfterStep(InfAfterStep {
                src,
                step,
                planned_direction,
            }),

            cetkaik_yhuap_move_candidates::PureMove::NonTamMoveFromHopZuo { color, prof, dest } => {
                Self::NormalMove(NormalMove::NonTamMoveFromHopZuo { color, prof, dest })
            }

            cetkaik_yhuap_move_candidates::PureMove::NonTamMoveSrcDst {
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

pub mod binary {
    #[test]
    fn it_works() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut hit = 0;
        let mut miss = 0;
        for _ in 0..=0xf_ffff_u32 {
            let i = rng.gen();
            match (
                InfAfterStep::from_binary(i),
                NormalMove::from_binary(i),
                AfterHalfAcceptance::from_binary(i),
            ) {
                (Err(_), Err(_), Err(_)) => {
                    miss += 1;
                }
                (Err(_), Err(_), Ok(after)) => {
                    hit += 1;
                    assert_eq!(after.to_binary(), i);
                }
                (Err(_), Ok(normal), Err(_)) => {
                    hit += 1;
                    assert_eq!(normal.to_binary(), i);
                }
                (Ok(inf), Err(_), Err(_)) => {
                    hit += 1;
                    assert_eq!(inf.to_binary(), i);
                }
                _ => panic!("Ambiguous mapping detected!!!!!"),
            }
        }
        println!(
            "hit: {}, miss: {}, hit ratio: {}",
            hit,
            miss,
            f64::from(hit) / (f64::from(hit) + f64::from(miss))
        );
    }

    use cetkaik_core::Profession;
    const fn prof_to_bin(p: Profession) -> i8 {
        match p {
            Profession::Nuak1 => -1,
            Profession::Kauk2 => 1,
            Profession::Gua2 => 2,
            Profession::Kaun1 => 3,
            Profession::Dau2 => 4,
            Profession::Maun1 => 5,
            Profession::Kua2 => 6,
            Profession::Tuk2 => 7,
            Profession::Uai1 => 8,
            Profession::Io => 9,
        }
    }

    const fn bin_to_prof(a: i8) -> Result<Profession, &'static str> {
        match a {
            -1 => Ok(Profession::Nuak1),
            1 => Ok(Profession::Kauk2),
            2 => Ok(Profession::Gua2),
            3 => Ok(Profession::Kaun1),
            4 => Ok(Profession::Dau2),
            5 => Ok(Profession::Maun1),
            6 => Ok(Profession::Kua2),
            7 => Ok(Profession::Tuk2),
            8 => Ok(Profession::Uai1),
            9 => Ok(Profession::Io),
            _ => Err("Invalid profession"),
        }
    }

    use num::traits::FromPrimitive;

    enum_from_primitive! {
        #[derive(Debug, PartialEq, Eq, Copy, Hash, Clone)]
        #[repr(u8)]
        pub enum Tag {
            /// 1
            NonTamMoveSrcDst = 1,

            /// 2
            NonTamMoveSrcStepDstFinite = 2,

            /// 3
            TamMoveNoStep = 3,

            /// 4
            TamMoveStepsDuringFormer = 4,

            /// 5
            TamMoveStepsDuringLatter = 5,

            /// 6
            InfAfterStep = 6,

            /// 7
            AfterHalfAcceptance = 7,
        }
    }

    impl Binary for InfAfterStep {
        fn to_binary(&self) -> u32 {
            Bag {
                src: Some(self.src),
                step: Some(self.step),
                first_dest: None,
                second_dest: Some(self.planned_direction),
                tag: Tag::InfAfterStep,
            }
            .to_binary()
        }
        fn from_binary(a: u32) -> Result<Self, &'static str> {
            match Bag::from_binary(a)? {
                Bag {
                    src: Some(src),
                    step: Some(step),
                    first_dest: None,
                    second_dest: Some(planned_direction),
                    tag: Tag::InfAfterStep,
                } => Ok(Self {
                    src,
                    step,
                    planned_direction,
                }),
                _ => Err("cannot interpret the input as an InfAfterStep"),
            }
        }
    }

    impl Binary for AfterHalfAcceptance {
        fn to_binary(&self) -> u32 {
            Bag {
                src: None,
                step: None,
                first_dest: None,
                second_dest: self.dest,
                tag: Tag::AfterHalfAcceptance,
            }
            .to_binary()
        }
        fn from_binary(a: u32) -> Result<Self, &'static str> {
            match Bag::from_binary(a)? {
                Bag {
                    src: None,
                    step: None,
                    first_dest: None,
                    second_dest: dest,
                    tag: Tag::AfterHalfAcceptance,
                } => Ok(Self { dest }),
                _ => Err("cannot interpret the input as AfterHalfAcceptance"),
            }
        }
    }

    impl Binary for NormalMove {
        fn to_binary(&self) -> u32 {
            match *self {
                NormalMove::NonTamMoveSrcDst { src, dest } => Bag {
                    src: Some(src),
                    step: None,
                    first_dest: None,
                    second_dest: Some(dest),
                    tag: Tag::NonTamMoveSrcDst,
                },
                NormalMove::NonTamMoveSrcStepDstFinite { src, step, dest } => Bag {
                    src: Some(src),
                    step: Some(step),
                    first_dest: None,
                    second_dest: Some(dest),
                    tag: Tag::NonTamMoveSrcStepDstFinite,
                },
                NormalMove::NonTamMoveFromHopZuo { color, prof, dest } => {
                    use cetkaik_core::Color::{Huok2, Kok1};

                    #[allow(clippy::cast_sign_loss)]
                    let prof: u32 = (prof_to_bin(prof) as u8).into();
                    let color: u32 = match color {
                        Kok1 => 0,
                        Huok2 => 1,
                    };
                    let dest: u32 = to_7bit_(Some(dest)).into();
                    let tag = 0;
                    return prof | (color << 8) | (dest << 21) | (tag << 28);
                }
                NormalMove::TamMoveNoStep {
                    src,
                    first_dest,
                    second_dest,
                } => Bag {
                    src: Some(src),
                    first_dest: Some(first_dest),
                    second_dest: Some(second_dest),
                    step: None,
                    tag: Tag::TamMoveNoStep,
                },
                NormalMove::TamMoveStepsDuringFormer {
                    src,
                    first_dest,
                    second_dest,
                    step,
                } => Bag {
                    src: Some(src),
                    first_dest: Some(first_dest),
                    second_dest: Some(second_dest),
                    step: Some(step),
                    tag: Tag::TamMoveStepsDuringFormer,
                },
                NormalMove::TamMoveStepsDuringLatter {
                    src,
                    first_dest,
                    second_dest,
                    step,
                } => Bag {
                    src: Some(src),
                    first_dest: Some(first_dest),
                    second_dest: Some(second_dest),
                    step: Some(step),
                    tag: Tag::TamMoveStepsDuringLatter,
                },
            }
            .to_binary()
        }
        fn from_binary(v: u32) -> Result<Self, &'static str> {
            use std::convert::TryInto;
            let tag: u8 = ((v & (15 << 28)) >> 28).try_into().unwrap();
            if tag == 0
            /* NormalMove::NonTamMoveFromHopZuo */
            {
                use cetkaik_core::Color::{Huok2, Kok1};
                let prof: u8 = (v & 255).try_into().unwrap();

                #[allow(clippy::cast_possible_wrap)]
                let prof = bin_to_prof(prof as i8)?;
                let color: u8 = ((v & (1 << 8)) >> 8).try_into().unwrap();
                let color = match color {
                    0 => Kok1,
                    1 => Huok2,
                    _ => return Err("Invalid color"),
                };

                let zero = (v & (0b1111_1111_1111 << 9)) >> 9;

                if zero != 0 {
                    return Err("Expected zero bits in NormalMove::NonTamMoveFromHopZuo");
                }

                let dest: u8 = ((v & (127 << 21)) >> 21).try_into().unwrap();
                let dest = from_7bit_(dest)?.ok_or("Invalid destination")?;
                Ok(Self::NonTamMoveFromHopZuo { color, prof, dest })
            } else {
                Ok(match Bag::from_binary(v)? {
                    Bag {
                        src: Some(src),
                        first_dest: Some(first_dest),
                        second_dest: Some(second_dest),
                        step: Some(step),
                        tag: Tag::TamMoveStepsDuringLatter,
                    } => Self::TamMoveStepsDuringLatter {
                        src,
                        first_dest,
                        second_dest,
                        step,
                    },

                    Bag {
                        src: Some(src),
                        first_dest: Some(first_dest),
                        second_dest: Some(second_dest),
                        step: Some(step),
                        tag: Tag::TamMoveStepsDuringFormer,
                    } => Self::TamMoveStepsDuringFormer {
                        src,
                        first_dest,
                        second_dest,
                        step,
                    },
                    Bag {
                        src: Some(src),
                        first_dest: Some(first_dest),
                        second_dest: Some(second_dest),
                        step: None,
                        tag: Tag::TamMoveNoStep,
                    } => Self::TamMoveNoStep {
                        src,
                        first_dest,
                        second_dest,
                    },

                    Bag {
                        src: Some(src),
                        step: None,
                        first_dest: None,
                        second_dest: Some(dest),
                        tag: Tag::NonTamMoveSrcDst,
                    } => Self::NonTamMoveSrcDst { src, dest },
                    Bag {
                        src: Some(src),
                        step: Some(step),
                        first_dest: None,
                        second_dest: Some(dest),
                        tag: Tag::NonTamMoveSrcStepDstFinite,
                    } => Self::NonTamMoveSrcStepDstFinite { src, step, dest },
                    _ => return Err("Cannot interpret the input as a NormalMove"),
                })
            }
        }
    }

    use super::{absolute, AfterHalfAcceptance, InfAfterStep, NormalMove};

    struct Bag {
        src: Option<absolute::Coord>,
        step: Option<absolute::Coord>,
        first_dest: Option<absolute::Coord>,
        second_dest: Option<absolute::Coord>,
        tag: Tag,
    }

    fn to_7bit_(c: Option<absolute::Coord>) -> u8 {
        match c {
            None => 127,
            Some(c) => to_7bit(c),
        }
    }

    fn to_7bit(c: absolute::Coord) -> u8 {
        use std::convert::TryInto;
        let [row, col] = cetkaik_core::perspective::to_relative_coord(
            c,
            cetkaik_core::perspective::Perspective::IaIsDownAndPointsUpward,
        );

        (row * 9 + col).try_into().unwrap()
    }

    fn from_7bit_(a: u8) -> Result<Option<absolute::Coord>, &'static str> {
        Ok(if a == 127 { None } else { Some(from_7bit(a)?) })
    }

    fn from_7bit(id: u8) -> Result<absolute::Coord, &'static str> {
        if id >= 81 {
            return Err("Invalid index of a square");
        }

        let row = id / 9;
        let col = id % 9;

        Ok(cetkaik_core::perspective::to_absolute_coord(
            [row.into(), col.into()],
            cetkaik_core::perspective::Perspective::IaIsDownAndPointsUpward,
        ))
    }

    /// * bit 28-31: `tag` (See [`Tag`](enum.Tag.html) for details)
    ///
    /// If `tag` is nonzero:
    /// * bit 0-6: `src`
    /// * bit 7-13: `step`
    /// * bit 14-20: `first_dest`
    /// * bit 21-27: `second_dest` / `dest` / `planned_direction`
    ///
    /// If `tag` is zero, then the encoded value is `NormalMove::NonTamMoveFromHopZuo`;   
    /// * bit 0-7: `prof`
    /// * bit 8: `color` (`0` if red; `1` if black)
    /// * bit 9-20: zero bits
    /// * bit 21-27: `dest`
    pub trait Binary
    where
        Self: std::marker::Sized,
    {
        fn to_binary(&self) -> u32;
        fn from_binary(b: u32) -> Result<Self, &'static str>;

        #[must_use]
        fn random_choice() -> Self {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            loop {
                let i = rng.gen();
                match Self::from_binary(i) {
                    Ok(a) => return a,
                    Err(_) => continue,
                }
            }
        }
    }

    impl Binary for Bag {
        fn to_binary(&self) -> u32 {
            let src: u32 = to_7bit_(self.src).into();
            let step: u32 = to_7bit_(self.step).into();
            let first_dest: u32 = to_7bit_(self.first_dest).into();
            let second_dest: u32 = to_7bit_(self.second_dest).into();
            let tag = self.tag as u8;
            if tag > 15 {
                panic!("tag too large");
            }
            let tag: u32 = tag.into();
            src | (step << 7) | (first_dest << 14) | (second_dest << 21) | (tag << 28)
        }

        fn from_binary(v: u32) -> Result<Self, &'static str> {
            use std::convert::TryInto;
            let src: u8 = (v & 127).try_into().unwrap();
            let step: u8 = ((v & (127 << 7)) >> 7).try_into().unwrap();
            let first_dest: u8 = ((v & (127 << 14)) >> 14).try_into().unwrap();
            let second_dest: u8 = ((v & (127 << 21)) >> 21).try_into().unwrap();
            let tag: u8 = ((v & (15 << 28)) >> 28).try_into().unwrap();

            let src = from_7bit_(src)?;
            let step = from_7bit_(step)?;
            let first_dest = from_7bit_(first_dest)?;
            let second_dest = from_7bit_(second_dest)?;

            Ok(Self {
                src,
                step,
                first_dest,
                second_dest,
                tag: Tag::from_u8(tag).ok_or("invalid tag")?,
            })
        }
    }
}
