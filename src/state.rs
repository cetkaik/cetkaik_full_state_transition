use super::{absolute, state, IfTaxot, Rate, Season};
/// Normal state. ／一番普通の状態。
#[derive(Clone, Debug)]
pub struct A {
    pub f: absolute::Field,
    pub whose_turn: absolute::Side,
    pub season: Season,
    pub ia_owner_s_score: i32,
    pub rate: Rate,
    pub tam_has_moved_previously: bool,
}

/// This is the state after the user has stepped over a piece and has cast the sticks so that the user can play to make an infinite movement from there. Seeing the sticks, the user is supposed to decide the final location and send it (`AfterHalfAcceptance`) to the server.
/// ／踏越え後の無限移動をユーザーが行い、それに対して投げ棒で判定した後の状態。投げ棒を見て、ユーザーは最終的な移動場所をCに対しこれから送りつける。
#[derive(Clone, Debug)]
pub struct C {
    pub c: CWithoutCiurl,
    pub ciurl: i32,
}

/// Same as `C`, except that the ciurl is not mentioned.
/// ／`C` から投げ棒の値を除いたやつ。
#[derive(Clone, Debug)]
pub struct CWithoutCiurl {
    pub f: absolute::Field,
    pub whose_turn: absolute::Side,
    pub flying_piece_src: absolute::Coord,
    pub flying_piece_step: absolute::Coord,
    pub season: Season,
    pub ia_owner_s_score: i32,
    pub rate: Rate,
}

/// The water entry cast (if any) is now over, and thus the piece movement is now fully completed. However, I still haven't resolved whether a hand exists. If so, I must ask the user to choose whether to end the season or not.
/// ／入水判定も終わり、駒を完全に動かし終わった。しかしながら、「役が存在していて再行・終季をユーザーに訊く」を発生させるか否かをまだ解決していない。そんな状態。
#[derive(Clone, Debug)]
pub struct HandNotResolved {
    pub f: absolute::Field,
    pub whose_turn: absolute::Side,
    pub season: Season,
    pub ia_owner_s_score: i32,
    pub rate: Rate,
    pub i_have_moved_tam_in_this_turn: bool,
    pub previous_a_side_hop1zuo1: Vec<absolute::NonTam2Piece>,
    pub previous_ia_side_hop1zuo1: Vec<absolute::NonTam2Piece>,
    pub kut2tam2_happened: bool,
}

/// Converting `HandNotResolved` into `HandResolved` with `resolve` tells you whether a new hand was created. If so, the `HandExists` variant is taken; if not, the `NeitherTymokNorTaxot` is taken.
/// ／`HandNotResolved` を `resolve` でこの型に変換することによって、『役は発生しなかったぞ』であるのか、それとも『役は発生しており、したがって【再行ならこの `A` に至る】【終季ならこの `Probabilistic<state::A>` に至る（どちらが先手になるかは鯖のみぞ知るので `Probabilistic`）】』のどちらであるかを知ることができる。撃皇が役を構成するかどうかによってここの処理は変わってくるので、
/// `resolve` は `Config` を要求する。
pub enum HandResolved {
    NeitherTymokNorTaxot(state::A),
    HandExists {
        if_tymok: state::A,
        if_taxot: IfTaxot,
    },
}
