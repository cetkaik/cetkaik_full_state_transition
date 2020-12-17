pub const CIURL_DISTRIBUTION: [(i32, f64); 6] = [
    (0, 1.0 / 32.0),
    (1, 5.0 / 32.0),
    (2, 10.0 / 32.0),
    (3, 10.0 / 32.0),
    (4, 5.0 / 32.0),
    (5, 1.0 / 32.0),
];

#[derive(Debug, Clone)]
pub struct Probabilistic<T>(Vec<(T, f64)>);

impl<T> Probabilistic<T> {
    pub fn to_inner(self) -> Vec<(T, f64)> {
        self.0
    }

    pub fn map<B, F>(self, f: F) -> Probabilistic<B>
    where
        F: Fn(T) -> B,
    {
        Probabilistic(self.0.into_iter().map(|(t, n)| (f(t), n)).collect())
    }
}
