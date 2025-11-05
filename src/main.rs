use std::collections::{HashMap, HashSet};
mod hex_to_base64;
mod xor;
use hex_to_base64::*;
use xor::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_1_3() {
        let sample = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let decoded_sample = hex::decode(sample).unwrap();
        let mut results = decode_single_character_xor(&decoded_sample);
        results.sort_by(|a, b| a.0.total_cmp(&b.0));

        for i in 0..3 {
            println!("{:?}", results.get(i));
        }
        assert_eq!(true, true);
    }

    #[test]
    fn test_1_4() {
        let content = fs::read_to_string("data_1_4.txt").unwrap();
        let mut agg_results: Vec<(f64, char, String)> = Vec::new();
        for l in content.lines() {
            let hex_decoded = hex::decode(l).unwrap();
            let mut results = decode_single_character_xor(&hex_decoded);
            results.sort_by(|a, b| a.0.total_cmp(&b.0));
            agg_results.append(&mut results);
        }
        agg_results.sort_by(|a, b| a.0.total_cmp(&b.0));

        for i in 0..20 {
            println!("{:?}", agg_results.get(i));
        }
        assert_eq!(true, true);
    }
}
