use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

pub fn english_letter_distribution() -> LazyLock<HashMap<char, f64>> {
    LazyLock::new(|| {
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
    })
}

pub fn rmse(a: &HashMap<char, f64>, b: &HashMap<char, f64>) -> f64 {
    let akeys: HashSet<&char> = a.keys().collect();
    let bkeys: HashSet<&char> = b.keys().collect();
    let union_keys = akeys.union(&bkeys);
    let mut len = 0f64;
    let squared_deltas = union_keys.fold(0f64, |acc, &k| {
        let av = a.get(k).unwrap_or(&0f64);
        let bv = b.get(k).unwrap_or(&0f64);
        len += 1f64;
        acc + (av - bv).powi(2)
    });
    (squared_deltas / len).powf(0.5)
}

pub fn count_sample(sample: &[char]) -> HashMap<char, f64> {
    let normal = 1f64 / sample.len() as f64;
    let mut result: HashMap<char, f64> = HashMap::new();
    sample.iter().for_each(|u| {
        result
            .entry(u.to_ascii_lowercase())
            .and_modify(|f| *f += normal)
            .or_insert(normal);
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_set() {
        let sample: Vec<char> = "aabcd".chars().collect();
        println!("{:?}", sample);
        assert_eq!(count_sample(&sample).get(&'a'), Some(&0.4f64));
    }

    #[test]
    fn test_mean_squared_compare() {
        let a = HashMap::from([('a', 0f64), ('b', 0f64), ('c', 0f64)]);
        let b = HashMap::from([('a', 1f64), ('b', 2f64), ('c', 3f64)]);
        let result: f64 = rmse(&a, &b);
        assert_eq!(result, (14f64 / 3f64).powf(0.5));
    }
}
