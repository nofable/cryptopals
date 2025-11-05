use base64::prelude::*;
mod letters;
use letters::*;

pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(i, j)| i ^ j).collect()
}

pub fn repeating_key_xor_strs(sample: &str, key: &str) -> String {
    let sample_bytes = sample.as_bytes().to_vec();
    let key_bytes = key.as_bytes().to_vec();
    let output_bytes = repeating_key_xor(&sample_bytes, &key_bytes);
    String::from_utf8(output_bytes).unwrap()
}

pub fn repeating_key_xor_hexs(sample: &str, key: &str) -> String {
    let decoded_sample = hex::decode(sample).unwrap();
    let decoded_key = hex::decode(key).unwrap();
    let output_bytes = repeating_key_xor(&decoded_sample, &decoded_key);
    hex::encode(output_bytes)
}

pub fn repeating_key_xor(sample: &[u8], key: &[u8]) -> Vec<u8> {
    let sl = sample.len();
    let kl = key.len();
    let div = sl / kl;
    let remain = sl % kl;
    let mut repeating_key = key.repeat(div);
    repeating_key.append(&mut key[0..remain].to_vec());
    xor_bytes(sample, &repeating_key)
}

pub fn decode_single_character_xor(sample: &[u8]) -> Vec<(f64, char, String)> {
    let chars: Vec<u8> = (32u8..=126u8).collect();
    let english_letter_dist = english_letter_distribution();
    // in the results vec,
    // f64 is the mean_squared score,
    // char is the XOR char
    // String is the decrypted string
    let mut results: Vec<(f64, char, String)> = Vec::new();
    for c in chars {
        let xored = repeating_key_xor(sample, &[c]);
        let distribution = count_sample(&xored);
        let score = mean_squared_compare(&distribution, &english_letter_dist);
        if let Ok(s) = String::from_utf8(xored) {
            results.push((score, c as char, s));
        }
    }
    results.sort_by(|a, b| a.0.total_cmp(&b.0));
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_case() {
        let input_a = "1c0111001f010100061a024b53535009181c";
        let input_b = "686974207468652062756c6c277320657965";
        let expected: String = "746865206b696420646f6e277420706c6179".into();
        assert_eq!(repeating_key_xor_hexs(input_a, input_b), expected);
    }

    #[test]
    #[should_panic]
    fn test_fail_not_hex() {
        let input_a = "$%&111001f010100061a024b53535009181c";
        let input_b = "686974207468652062756c6c277320657965";
        repeating_key_xor_hexs(input_a, input_b);
    }

    #[test]
    fn test_repeating_key_xor() {
        let sample = "Cooking MC's like a pound of bacon";
        let key = "X";
        assert_eq!(
            repeating_key_xor_strs(sample, key),
            "\u{1b}77316?x\u{15}\u{1b}\u{7f}+x413=x9x(7-6<x7>x:9;76"
        );
    }
}
