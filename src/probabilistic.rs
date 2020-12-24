pub enum Probabilistic<T> {
    Pure(T),
    Water {
        failure: T,
        success: T,
    },
    Sticks {
        s0: T,
        s1: T,
        s2: T,
        s3: T,
        s4: T,
        s5: T,
    },
    WhoGoesFirst {
        ia_first: T,
        a_first: T,
    },
}

impl<T: Clone> Into<Prob<(T, Option<usize>)>> for Probabilistic<T> {
    fn into(self) -> Prob<(T, Option<usize>)> {
        match self {
            // Since this function does not contain any info on how who-goes-first gets decided,
            // to consistently display the ciurls, the implementer must, independent from this function, 
            // consider how who-goes-first gets decided.
            // 先手を決定する手段についてはこの関数で定めないので、
            // 投げ棒を一貫して表示するには、この関数と独立で実装側で先手決定手段を考慮しなければならない。
            Probabilistic::WhoGoesFirst { ia_first, a_first } => {
                vec![
                    ((ia_first, None), 1.0 / 2.0),
                    ((a_first, None), 1.0 / 2.0),
                ]
            }
            Probabilistic::Pure(t) => vec![((t, None), 1.0)],
            Probabilistic::Water { failure, success } => {
                vec![
                    ((failure.clone(), Some(0)), 1.0 / 32.0),
                    ((failure.clone(), Some(1)), 5.0 / 32.0),
                    ((failure.clone(), Some(2)), 10.0 / 32.0),
                    ((success.clone(), Some(3)), 10.0 / 32.0),
                    ((success.clone(), Some(4)), 5.0 / 32.0),
                    ((success.clone(), Some(5)), 1.0 / 32.0),
                ]
            }
            Probabilistic::Sticks {
                s0,
                s1,
                s2,
                s3,
                s4,
                s5,
            } => {
                vec![
                    ((s0, Some(0)), 1.0 / 32.0),
                    ((s1, Some(1)), 5.0 / 32.0),
                    ((s2, Some(2)), 10.0 / 32.0),
                    ((s3, Some(3)), 10.0 / 32.0),
                    ((s4, Some(4)), 5.0 / 32.0),
                    ((s5, Some(5)), 1.0 / 32.0),
                ]
            }
        }
    }
}

type Prob<T> = Vec<(T, f64)>;
