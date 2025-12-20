/// Computes the Hamming distance (number of differing bits)
/// between two equally-sized byte slices.
///
/// # Panics
/// Panics if the slices have different lengths.
pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b)
        .map(|(a_byte, b_byte)| (a_byte ^ b_byte).count_ones() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()),
            37
        );
    }
}
