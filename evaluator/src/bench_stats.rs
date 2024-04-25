#[derive(Clone, Debug, Default)]
pub struct BenchStats {
    pub(crate) max: f64,
    pub(crate) mean: f64,
    pub(crate) min: f64,
    pub(crate) sd: f64,
}

impl BenchStats {
    pub(crate) fn new(data: &[f64]) -> Self {
        let mut iter = data.iter().copied();
        let (mut min, mut max, mut sum) = if let Some(value) = iter.next() {
            (value, value, value)
        } else {
            return Self::default();
        };
        for value in data.iter().copied() {
            max = max.max(value);
            min = min.min(value);
            sum += value;
        }
        let len: f64 = u32::try_from(data.len()).unwrap().into();
        let mean = sum / len;
        Self {
            max,
            min,
            mean,
            sd: sd(data, len, mean),
        }
    }
}

fn sd(data: &[f64], len: f64, mean: f64) -> f64 {
    let sum = data
        .iter()
        .map(|value| {
            let diff = mean - value;
            diff * diff
        })
        .sum::<f64>();
    (sum / len).sqrt()
}
