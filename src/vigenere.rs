use crate::hamming::*;
use crate::transposer::*;
use crate::xor::*;
use base64::{Engine, prelude::BASE64_STANDARD};
use std::fs;

pub fn vigenere(
    input_file: String,
    max_hamming: usize,
    hamming_sample_size: usize,
    no_of_results: usize,
) {
    let content = fs::read_to_string(input_file).unwrap().replace("\n", "");
    let decoded = BASE64_STANDARD.decode(content).unwrap();
    let mut results: Vec<(f64, usize)> = Vec::new();

    // guess at the key size by evaluating the lowest hamming_distance, normalized by keysize
    for i in 2..max_hamming {
        if decoded.len() > i * hamming_sample_size {
            let mut slices: Vec<&[u8]> = Vec::new();
            let mut normalized_hamming_distances: Vec<f64> = Vec::new();
            for j in 1..=hamming_sample_size {
                let slice = &decoded[(i * (j - 1))..(i * j)];
                if !slices.is_empty() {
                    let distance = hamming_distance(slices.last().unwrap(), slice);
                    normalized_hamming_distances.push(distance as f64 / i as f64);
                }
                slices.push(slice);
            }
            let average = normalized_hamming_distances
                .iter()
                .fold(0.0, |result, item| {
                    result + (item / normalized_hamming_distances.len() as f64)
                });
            results.push((average, i));
        }
    }

    results.sort_by(|a, b| a.0.total_cmp(&b.0));
    // iterate through the top results
    for (_, keysize) in &results[0..no_of_results] {
        let blocks: Vec<Vec<u8>> = decoded
            .clone()
            .chunks(*keysize)
            .map(|block| block.to_vec())
            .collect();

        // transpose blocks
        let transposed = transpose(blocks);
        let mut key_chars: Vec<char> = Vec::new();
        // single char XOR
        for tblock in transposed {
            // histogram for letter frequency
            let mut scx_results = decode_single_character_xor(&tblock);
            scx_results.sort_by(|a, b| a.0.total_cmp(&b.0));
            let top_result = scx_results.first().unwrap();
            key_chars.push(top_result.1);
        }
        let key_string: String = key_chars.iter().collect();
        println!("final key is '{}'", key_string);
        let final_result =
            repeating_key_xor_strs(&String::from_utf8(decoded.clone()).unwrap(), &key_string);
        println!("Final result is {}", final_result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_6() {
        let result = vigenere("data_1_6.txt".into(), 40, 4, 3);
    }
}
