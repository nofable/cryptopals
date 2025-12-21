use crate::errors::CryptopalsError;
use crate::hamming::*;
use crate::transposer::*;
use crate::xor::*;
use base64::{Engine, prelude::BASE64_STANDARD};
use std::fs;

pub struct VigenereResult {
    pub key: String,
    pub plaintext: String,
}

pub fn vigenere(
    input_file: &str,
    max_hamming: usize,
    hamming_sample_size: usize,
    no_of_results: usize,
) -> Result<Vec<VigenereResult>, CryptopalsError> {
    let content = fs::read_to_string(input_file)?.replace("\n", "");
    let decoded = BASE64_STANDARD.decode(content)?;
    let mut results: Vec<VigenereResult> = Vec::with_capacity(no_of_results);

    let mut hamming_results: Vec<(f64, usize)> = Vec::new();

    // guess at the key size by evaluating the lowest hamming_distance, normalized by keysize
    for i in 2..max_hamming {
        if decoded.len() > i * hamming_sample_size {
            let mut prev = None;
            let mut average = 0.0;

            for chunk in decoded.chunks(i).take(hamming_sample_size) {
                if let Some(p) = prev {
                    average += (hamming_distance(p, chunk)? as f64 / i as f64)
                        / (hamming_sample_size as f64 - 1f64);
                }
                prev = Some(chunk);
            }
            hamming_results.push((average, i));
        }
    }

    hamming_results.sort_by(|a, b| a.0.total_cmp(&b.0));

    // iterate through the top results
    for (_, keysize) in hamming_results.iter().take(no_of_results) {
        let blocks: Vec<&[u8]> = decoded.chunks(*keysize).collect();

        // transpose blocks
        let transposed = transpose(&blocks)?;
        let mut key_chars: Vec<char> = Vec::with_capacity(transposed.len());
        // single char XOR
        for tblock in transposed {
            // histogram for letter frequency
            let mut scx_results = decode_single_character_xor(&tblock)?;
            scx_results.sort_by(|a, b| a.0.total_cmp(&b.0));
            if let Some(top_result) = scx_results.first() {
                key_chars.push(top_result.1);
            }
        }
        let key_string: String = key_chars.iter().collect();
        let final_result =
            repeating_key_xor_strs(&String::from_utf8(decoded.clone())?, &key_string)?;
        results.push(VigenereResult {
            key: key_string,
            plaintext: final_result,
        })
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_6() {
        let results = vigenere("data_1_6.txt", 40, 4, 3).unwrap();
        let top = &results[2];
        assert!(top.key.contains("Terminator")); // known key part
        assert!(top.plaintext.contains("Play that funky music")); // expected text snippet
    }
}
