use std::collections::HashMap;
use std::sync::LazyLock;

pub static ENGLISH_LETTER_DISTRIBUTION: LazyLock<HashMap<char, f64>> = LazyLock::new(|| {
    HashMap::from([
        ('a', 0.08167),
        ('b', 0.01492),
        ('c', 0.02782),
        ('d', 0.04253),
        ('e', 0.12702),
        ('f', 0.02228),
        ('g', 0.02015),
        ('h', 0.06094),
        ('i', 0.06966),
        ('j', 0.00153),
        ('k', 0.00772),
        ('l', 0.04025),
        ('m', 0.02406),
        ('n', 0.06749),
        ('o', 0.07507),
        ('p', 0.01929),
        ('q', 0.00095),
        ('r', 0.05987),
        ('s', 0.06327),
        ('t', 0.09056),
        ('u', 0.02758),
        ('v', 0.00978),
        ('w', 0.02360),
        ('x', 0.00150),
        ('y', 0.01974),
        ('z', 0.00074),
        (' ', 0.2),
    ])
});

// Root Mean Squared Error
pub fn rmse(a: &HashMap<char, f64>, b: &HashMap<char, f64>) -> f64 {
    let mut cnt = 0f64;
    let mut sum = 0f64;

    for (ac, af) in a {
        let bf = b.get(ac).unwrap_or(&0f64);
        sum += (af - bf).powi(2);
        cnt += 1.0;
    }

    for (bc, bf) in b {
        if !a.contains_key(bc) {
            sum += bf.powi(2);
            cnt += 1.0;
        }
    }
    (sum / cnt).sqrt()
}

pub fn count_sample(sample: &[char]) -> HashMap<char, f64> {
    if sample.is_empty() {
        return HashMap::new();
    }
    let normal = 1f64 / sample.len() as f64;
    let mut result = HashMap::new();
    for c in sample {
        result
            .entry(c.to_ascii_lowercase())
            .and_modify(|f| *f += normal)
            .or_insert(normal);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_set() {
        let sample: Vec<char> = "aabcd".chars().collect();
        let dist = count_sample(&sample);
        let value = dist.get(&'a').unwrap();
        assert!((value - 0.4).abs() < 1e-9);
    }

    #[test]
    fn test_rmse() {
        let a = HashMap::from([('a', 0f64), ('b', 0f64), ('c', 0f64)]);
        let b = HashMap::from([('a', 1f64), ('b', 2f64), ('c', 3f64)]);
        let result: f64 = rmse(&a, &b);
        assert!(((14f64 / 3f64).sqrt() - result).abs() < 1e-9);
    }
}
