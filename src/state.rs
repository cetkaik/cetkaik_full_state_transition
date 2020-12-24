use super::*;
/// 一番普通の状態。定常状態。
#[derive(Clone, Debug)]
pub struct A {
    pub f: absolute::Field,
    pub whose_turn: absolute::Side,
    pub season: Season,
    pub ia_owner_s_score: i32,
    pub rate: Rate,
    pub tam_has_moved_previously: bool,
}

/// 踏越え後の無限移動をユーザーが行い、それに対して裁で判定した後の状態。
/// 裁を見て、ユーザーは最終的な移動場所をCに対しこれから送りつける。
#[derive(Clone, Debug)]
pub struct C {
    pub c: CWithoutCiurl,
    pub ciurl: i32,
}

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

/// 入水判定も終わり、駒を完全に動かし終わった。
/// しかしながら、「役が存在していて再行・終季をユーザーに訊く」を
/// 発生させるか否かをまだ解決していない。
/// そんな状態。
#[derive(Clone, Debug)]
pub struct HandNotResolved {
    pub f: absolute::Field,
    pub whose_turn: absolute::Side,
    pub season: Season,
    pub ia_owner_s_score: i32,
    pub rate: Rate,
    pub tam_has_moved_previously: bool,
    pub previous_a_side_hop1zuo1: Vec<absolute::NonTam2Piece>,
    pub previous_ia_side_hop1zuo1: Vec<absolute::NonTam2Piece>,
    pub kut2tam2_happened: bool,
}

/// `state::HandNotResolved` を `resolve` でこの型に変換することによって、
/// 「役は発生しなかったぞ」 vs.
/// 「役は発生しており、したがって
/// * 再行ならこの `state::A` に至る
/// * 終季ならこの `Probabilistic<state::A>` に至る
/// （どちらが先手になるかは鯖のみぞ知るので `Probabilistic`）
/// 」のどちらであるかを知ることができる。
/// 撃皇が役を構成するかどうかによってここの処理は変わってくるので、
/// `Config` が要求されることになる。
pub enum HandResolved {
    NeitherTymokNorTaxot(state::A),
    HandExists {
        if_tymok: state::A,
        if_taxot: IfTaxot,
    },
}
