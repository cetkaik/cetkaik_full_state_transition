#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Scores {
    ia: i32,
    a: i32,
}

pub struct Victor(Option<cetkaik_core::absolute::Side>);

use cetkaik_core::absolute;
impl Scores {
    #[must_use]
    pub fn new() -> Scores {
        Scores { ia: 20, a: 20 }
    }

    #[must_use]
    pub fn ia(self) -> i32 {
        self.ia
    }

    #[must_use]
    pub fn a(self) -> i32 {
        self.a
    }

    pub fn edit(
        self,
        raw_score: i32,
        whose_turn: cetkaik_core::absolute::Side,
        rate: super::Rate,
    ) -> Result<Self, Victor> {
        let increment_in_ia_owner_s_score = match whose_turn {
            absolute::Side::IASide => 1,
            absolute::Side::ASide => -1,
        } * rate.num()
            * raw_score;

        let new_ia_owner_s_score = 0.max(40.min(self.ia + increment_in_ia_owner_s_score));
        if new_ia_owner_s_score == 40 {
            Err(Victor(Some(absolute::Side::IASide)))
        } else if new_ia_owner_s_score == 0 {
            Err(Victor(Some(absolute::Side::ASide)))
        } else {
            Ok(Scores {
                ia: new_ia_owner_s_score,
                a: 40 - new_ia_owner_s_score,
            })
        }
    }

    #[must_use]
    pub fn which_side_is_winning(self) -> Victor {
        match self.ia.cmp(&(40 - self.ia)) {
            std::cmp::Ordering::Greater => {
                Victor(Some(absolute::Side::IASide))
            }
            std::cmp::Ordering::Less =>Victor(Some(absolute::Side::ASide)),
            std::cmp::Ordering::Equal => Victor(None),
        }
    }
}
