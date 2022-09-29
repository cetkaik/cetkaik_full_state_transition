/// Describes the probability density due to the sticks cast.
/// ／投げ棒に由来する確率分布。
#[derive(Debug, Clone)]
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

impl<T: Clone> Probabilistic<T> {
    #[must_use]
    pub fn choose(self) -> (T, Option<usize>) {
        let prob: Prob<_> = self.into();
        prob.choose()
    }

    /// # Panics
    /// Panics when called while ciurl exists.
    #[must_use]
    pub fn choose_when_no_ciurl(self) -> T {
        let prob: Prob<_> = self.into();
        match prob.choose() {
            (t, None) => t,
            _ => panic!("ciurl exists; call `choose` instead."),
        }
    }
}

impl<T: Clone> From<Probabilistic<T>> for Prob<(T, Option<usize>)> {
    fn from(s: Probabilistic<T>) -> Self {
        match s {
            // Since this function does not contain any info on how who-goes-first gets decided,
            // to consistently display the ciurls, the implementer must, independent from this function,
            // consider how who-goes-first gets decided.
            // 先手を決定する手段についてはこの関数で定めないので、
            // 投げ棒を一貫して表示するには、この関数と独立で実装側で先手決定手段を考慮しなければならない。
            Probabilistic::WhoGoesFirst { ia_first, a_first } => Self(vec![
                ((ia_first, None), 1.0 / 2.0),
                ((a_first, None), 1.0 / 2.0),
            ]),
            Probabilistic::Pure(t) => Self(vec![((t, None), 1.0)]),
            Probabilistic::Water { failure, success } => Self(vec![
                ((failure.clone(), Some(0)), 1.0 / 32.0),
                ((failure.clone(), Some(1)), 5.0 / 32.0),
                ((failure, Some(2)), 10.0 / 32.0),
                ((success.clone(), Some(3)), 10.0 / 32.0),
                ((success.clone(), Some(4)), 5.0 / 32.0),
                ((success, Some(5)), 1.0 / 32.0),
            ]),
            Probabilistic::Sticks {
                s0,
                s1,
                s2,
                s3,
                s4,
                s5,
            } => Self(vec![
                ((s0, Some(0)), 1.0 / 32.0),
                ((s1, Some(1)), 5.0 / 32.0),
                ((s2, Some(2)), 10.0 / 32.0),
                ((s3, Some(3)), 10.0 / 32.0),
                ((s4, Some(4)), 5.0 / 32.0),
                ((s5, Some(5)), 1.0 / 32.0),
            ]),
        }
    }
}

/// Describes the general probability density. Note that this implementation assumes that the sum is exactly 1.
/// ／一般の確率分布。f64の和が厳密に1になることを前提としている。
#[readonly::make]
pub struct Prob<T>(pub Vec<(T, f64)>);

impl<T> Prob<T> {
    #[must_use]
    /// Expects a float within the range of 0 up to but not including 1.
    /// # Panics
    /// Panics if float is outside the range.
    pub fn choose_by_uniform_random_variable(self, rand: f64) -> T {
        assert!((0.0..1.0).contains(&rand), "Expects a float within the range of 0 up to but not including 1");
        let mut threshold = 0.0;
        for (t, prob) in self.0 {
            if (threshold..(threshold + prob)).contains(&rand) {
                return t;
            }
            threshold += prob;
        }
        panic!("Something went wrong in `choose_by_uniform_random_variable`")
    }

    #[must_use]
    pub fn choose(self) -> T {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        self.choose_by_uniform_random_variable(rng.gen())
    }
}
