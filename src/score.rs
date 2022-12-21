use cetkaik_fundamental::AbsoluteSide;
use serde::{Deserialize, Serialize};
/// Describes the scores that the two players have. Players each begin with 20 points, and loses when all the points are lost.
/// ／両プレイヤーが持つ得点を表す型。双方20点スタートであり、点が0点になると敗北。
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Scores {
    ia: i32,
    a: i32,
}

/// Describes who won the game. If `Victor(None)`, the game is a tie.
/// ／どちらが勝利したのかを表現する型。 `Victor(None)` であれば引き分け。
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Victor(pub Option<cetkaik_fundamental::AbsoluteSide>);

impl Default for Scores {
    fn default() -> Self {
        Self::new()
    }
}

impl Scores {
    #[must_use]
    pub const fn new() -> Self {
        Self { ia: 20, a: 20 }
    }

    #[must_use]
    pub const fn ia(self) -> i32 {
        self.ia
    }

    #[must_use]
    pub const fn a(self) -> i32 {
        self.a
    }

    pub fn edit(
        self,
        raw_score: i32,
        whose_turn: cetkaik_fundamental::AbsoluteSide,
        rate: super::Rate,
    ) -> Result<Self, Victor> {
        let increment_in_ia_owner_s_score = match whose_turn {
            AbsoluteSide::IASide => 1,
            AbsoluteSide::ASide => -1,
        } * rate.num()
            * raw_score;

        let new_ia_owner_s_score = 0.max(40.min(self.ia + increment_in_ia_owner_s_score));
        if new_ia_owner_s_score == 40 {
            Err(Victor(Some(AbsoluteSide::IASide)))
        } else if new_ia_owner_s_score == 0 {
            Err(Victor(Some(AbsoluteSide::ASide)))
        } else {
            Ok(Self {
                ia: new_ia_owner_s_score,
                a: 40 - new_ia_owner_s_score,
            })
        }
    }

    #[must_use]
    pub fn which_side_is_winning(self) -> Victor {
        match self.ia.cmp(&(40 - self.ia)) {
            std::cmp::Ordering::Greater => Victor(Some(AbsoluteSide::IASide)),
            std::cmp::Ordering::Less => Victor(Some(AbsoluteSide::ASide)),
            std::cmp::Ordering::Equal => Victor(None),
        }
    }
}
