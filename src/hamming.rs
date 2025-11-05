pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(i, j)| (i ^ j).count_ones() as usize)
        .sum()
}

pub fn hamming_distance_strs(a: &str, b: &str) -> usize {
    hamming_distance(a.as_bytes(), b.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            hamming_distance_strs("this is a test", "wokka wokka!!!"),
            37
        );
    }
}
