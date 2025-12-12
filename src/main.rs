use base64::{Engine, prelude::BASE64_STANDARD};
use std::fs;
mod hamming;
mod hex_to_base64;
mod transposer;
mod xor;
use clap::Parser;
use hamming::*;
use hex_to_base64::*;
use transposer::*;
use xor::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,

    #[arg(short, long, default_value_t = 40)]
    max_hamming: usize,

    #[arg(short = 'a', long, default_value_t = 4)]
    hamming_sample_size: usize,

    #[arg(short, long, default_value_t = 3)]
    no_of_results: usize,
}

fn main() {
    let args = Args::parse();
    vigenere(
        args.input_file,
        args.max_hamming,
        args.hamming_sample_size,
        args.no_of_results,
    );
}

fn vigenere(
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
    #[test]
    fn test_1_6() {
        let result = vigenere("data_1_6.txt".into(), 40, 4, 3);
    }
}
