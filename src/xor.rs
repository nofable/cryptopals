use crate::errors::CryptopalsError;
use crate::letters::*;

pub fn xor_bytes(a: &[u8], b: &[u8]) -> Result<Vec<u8>, CryptopalsError> {
    if a.len() != b.len() {
        return Err(CryptopalsError::ParametersLengthMismatch);
    }
    Ok(a.iter().zip(b.iter()).map(|(&i, &j)| i ^ j).collect())
}

pub fn repeating_key_xor_strs(sample: &str, key: &str) -> Result<String, CryptopalsError> {
    let bytes = repeating_key_xor(sample.as_bytes(), key.as_bytes())?;
    Ok(String::from_utf8(bytes)?)
}

#[allow(unused)]
pub fn repeating_key_xor_hexs(sample: &str, key: &str) -> Result<String, CryptopalsError> {
    let decoded_sample = hex::decode(sample)?;
    let decoded_key = hex::decode(key)?;
    let output_bytes = repeating_key_xor(&decoded_sample, &decoded_key)?;
    Ok(hex::encode(output_bytes))
}

pub fn repeating_key_xor(sample: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptopalsError> {
    let sl = sample.len();
    let kl = key.len();
    if sl == 0 {
        return Err(CryptopalsError::InvalidParameter(
            "Sample has no length".into(),
        ));
    }
    if kl == 0 {
        return Err(CryptopalsError::InvalidParameter(
            "Key has no length".into(),
        ));
    }
    let div = sl / kl;
    let remain = sl % kl;
    let mut repeating_key = key.repeat(div);
    repeating_key.append(&mut key[0..remain].to_vec());
    xor_bytes(sample, &repeating_key)
}

pub fn decode_single_character_xor(
    sample: &[u8],
) -> Result<Vec<(f64, char, String)>, CryptopalsError> {
    let chars: Vec<u8> = (32u8..=126u8).collect();
    // in the results vec,
    // f64 is the mean_squared score,
    // char is the XOR char
    // String is the decrypted string
    let mut results: Vec<(f64, char, String)> = Vec::new();
    for c in chars {
        let xored = repeating_key_xor(sample, &[c])?;
        let text = String::from_utf8(xored)?;
        let distribution = count_sample(&text.chars().collect::<Vec<char>>());
        let score = rmse(&distribution, &ENGLISH_LETTER_DISTRIBUTION);
        results.push((score, c as char, text));
    }
    results.sort_by(|a, b| a.0.total_cmp(&b.0));
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_success_case() {
        let input_a = "1c0111001f010100061a024b53535009181c";
        let input_b = "686974207468652062756c6c277320657965";
        let expected: String = "746865206b696420646f6e277420706c6179".into();
        assert_eq!(repeating_key_xor_hexs(input_a, input_b).unwrap(), expected);
    }

    #[test]
    #[should_panic]
    fn test_fail_not_hex() {
        let input_a = "$%&111001f010100061a024b53535009181c";
        let input_b = "686974207468652062756c6c277320657965";
        repeating_key_xor_hexs(input_a, input_b).unwrap();
    }

    #[test]
    fn test_repeating_key_xor() {
        let sample = "Cooking MC's like a pound of bacon";
        let key = "X";
        assert_eq!(
            repeating_key_xor_strs(sample, key).unwrap(),
            "\u{1b}77316?x\u{15}\u{1b}\u{7f}+x413=x9x(7-6<x7>x:9;76"
        );
    }

    #[test]
    fn test_cryptopals_1_3() {
        let sample = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let decoded_sample = hex::decode(sample).unwrap();
        let mut results = decode_single_character_xor(&decoded_sample).unwrap();
        results.sort_by(|a, b| a.0.total_cmp(&b.0));

        for i in 0..3 {
            println!("{:?}", results.get(i));
        }
        assert_eq!(true, true);
    }

    #[test]
    fn test_cryptopals_1_4() {
        let content = fs::read_to_string("data_1_4.txt").unwrap();
        let mut agg_results: Vec<(f64, char, String)> = Vec::new();
        for l in content.lines() {
            let hex_decoded = hex::decode(l).unwrap();
            let Ok(mut results) = decode_single_character_xor(&hex_decoded) else {
                println!("decode_single_character_xor failed on a line");
                continue;
            };
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
    fn test_cryptopals_1_5() {
        let treatment =
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let result = repeating_key_xor_strs(treatment, "ICE").unwrap();
        let hexed = hex::encode(result);
        assert_eq!(hexed, expected);
    }
}
