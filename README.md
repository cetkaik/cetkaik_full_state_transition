# cetkaik_full_state_transition

![](https://docs.rs/cetkaik_full_state_transition/badge.svg)

官定内の変種はなるべくサポートするように心がけています。「[はじめての人のためのパイグ将棋](https://docs.google.com/document/d/17_cfVKLX5tGPYYRp5DUjnc8LEBOCs3uwX7t9QhO0nCY/edit#)」に載っているやつだと、

| 変種 | フラグ |
| ---- | ----- |
| 「撃皇は役であるので、それのみで終季を達成できる」vs.「撃皇は役ではなく、即時減点」 | [`step_tam_is_a_hand`](https://docs.rs/cetkaik_full_state_transition/0.1.6/cetkaik_full_state_transition/struct.Config.html#structfield.step_tam_is_a_hand) |
| 「撃皇後に判定に失敗したときに撃皇が成立するか否か」| [`failure_to_complete_the_move_means_exempt_from_kut2_tam2`](https://docs.rs/cetkaik_full_state_transition/0.1.6/cetkaik_full_state_transition/struct.Config.html#structfield.failure_to_complete_the_move_means_exempt_from_kut2_tam2) |
| 「自分の番で皇を動かしながら結局皇の位置に変化がない」はただ自分が一手損するだけなので、罰さなくていいという流派がある | [`tam_mun_mok`](https://docs.rs/cetkaik_full_state_transition/0.1.6/cetkaik_full_state_transition/struct.Config.html#structfield.tam_mun_mok) を [`Consequence::Allowed`](https://docs.rs/cetkaik_full_state_transition/0.1.6/cetkaik_full_state_transition/enum.Consequence.html#variant.Allowed) に |
