mod hex_to_base64;
mod xor;
use hex_to_base64::*;
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
        for c in chars {
            let result = xor_bytes(&vec![c], &decoded_treatment);
            println!("{:?}", result);
        }
    }
}
