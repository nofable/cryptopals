use std::collections::{HashMap, HashSet};
mod hex_to_base64;
mod letters;
mod xor;
use hex_to_base64::*;
use letters::*;
use xor::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_2() {
        let treatment = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let chars: Vec<u8> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
            .as_bytes()
            .to_vec();
        let decoded_treatment = hex::decode(treatment).unwrap();

        // in the results vec,
        // f64 is the mean_squared score,
        // char is the XOR char
        // String is the decrypted string
        let mut results: Vec<(f64, char, String)> = Vec::new();
        for c in chars {
            let sample = xor_bytes(&vec![c; decoded_treatment.len()], &decoded_treatment);
            let distribution = count_sample(&sample);
            let english_letter_dist = english_letter_distribution();
            let score = mean_squared_compare(&distribution, &english_letter_dist);
            results.push((score, c as char, String::from_utf8(sample).unwrap()));
        }
        results.sort_by(|a, b| a.0.total_cmp(&b.0));
        for i in 0..3 {
            println!("{:?}", results.get(i));
        }
        assert_eq!(true, true);
    }
}
