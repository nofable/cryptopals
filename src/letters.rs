use std::collections::{HashMap, HashSet};

pub fn english_letter_distribution() -> HashMap<u8, f64> {
    HashMap::from([
        (b'a', 0.08167),
        (b'b', 0.01492),
        (b'c', 0.02782),
        (b'd', 0.04253),
        (b'e', 0.12702),
        (b'f', 0.02228),
        (b'g', 0.02015),
        (b'h', 0.06094),
        (b'i', 0.06966),
        (b'j', 0.00153),
        (b'k', 0.00772),
        (b'l', 0.04025),
        (b'm', 0.02406),
        (b'n', 0.06749),
        (b'o', 0.07507),
        (b'p', 0.01929),
        (b'q', 0.00095),
        (b'r', 0.05987),
        (b's', 0.06327),
        (b't', 0.09056),
        (b'u', 0.02758),
        (b'v', 0.00978),
        (b'w', 0.02360),
        (b'x', 0.00150),
        (b'y', 0.01974),
        (b'z', 0.00074),
        (b' ', 0.2),
    ])
}

pub fn mean_squared_compare(a: &HashMap<u8, f64>, b: &HashMap<u8, f64>) -> f64 {
    let akeys: HashSet<&u8> = a.keys().collect();
    let bkeys: HashSet<&u8> = b.keys().collect();
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

pub fn count_sample(sample: &Vec<u8>) -> HashMap<u8, f64> {
    let l = sample.len() as f64;
    let mut result: HashMap<u8, f64> = HashMap::new();
    sample.iter().for_each(|u| {
        let mut lower: u8 = *u;
        // check if it's a capital letter and lowercase it
        if (65..=(65 + 25)).contains(&lower) {
            lower = u + 25;
        }
        result
            .entry(lower)
            .and_modify(|f| *f += 1f64 / l)
            .or_insert(1f64 / l);
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_set() {
        let sample: Vec<u8> = "aabcd".bytes().collect();
        assert_eq!(count_sample(&sample).get(&b'a'), Some(&0.4f64));
    }

    #[test]
    fn test_mean_squared_compare() {
        let a = HashMap::from([(1, 0f64), (2, 0f64), (3, 0f64)]);
        let b = HashMap::from([(1, 1f64), (2, 2f64), (3, 3f64)]);
        let result: f64 = mean_squared_compare(&a, &b);
        assert_eq!(result, (14f64 / 3f64).powf(0.5));
    }
}
