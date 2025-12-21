use crate::errors::CryptopalsError;

/// Computes the Hamming distance (number of differing bits)
/// between two equally-sized byte slices.
///
/// # Panics
/// Panics if the slices have different lengths.
pub fn hamming_distance(a: &[u8], b: &[u8]) -> Result<usize, CryptopalsError> {
    if a.len() != b.len() {
        return Err(CryptopalsError::HammingLengthMismatch);
    }
    Ok(a.iter()
        .zip(b)
        .map(|(a_byte, b_byte)| (a_byte ^ b_byte).count_ones() as usize)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()).unwrap(),
            37
        );
    }

    #[test]
    #[should_panic]
    fn test_length_mismatch() {
        hamming_distance("blah".as_bytes(), "blahblah".as_bytes()).unwrap();
    }
}
