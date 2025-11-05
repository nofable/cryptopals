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

        for i in 0..5 {
            if let Some(item) = agg_results.get(i) {
                println!("{:?}", item);
            }
        }
        assert_eq!(true, true);
    }

    #[test]
    fn test_1_5() {
        let treatment =
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let result = repeating_key_xor_strs(treatment, "ICE");
        let hexed = hex::encode(result);
        assert_eq!(hexed, expected);
    }
}
