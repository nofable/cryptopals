use base64::prelude::*;

fn xor_bytes(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(i, j)| i ^ j).collect()
}

fn xor_str(a: &str, b: &str) -> String {
    let decoded_a = hex::decode(a).unwrap();
    let decoded_b = hex::decode(b).unwrap();
    let output_bytes = xor_bytes(decoded_a, decoded_b);
    hex::encode(output_bytes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_case() {
        let input_a = "1c0111001f010100061a024b53535009181c";
        let input_b = "686974207468652062756c6c277320657965";
        let expected : String = "746865206b696420646f6e277420706c6179".into();
        assert_eq!(xor_str(input_a, input_b), expected);
    }

    #[test]
    #[should_panic]
    fn test_fail_mismatch_len() {
        let input_a = "11001f010100061a024b53535009181c";
        let input_b = "686974207468652062756c6c277320657965";
        xor_str(input_a, input_b);
    }

    #[test]
    #[should_panic]
    fn test_fail_not_hex() {
        let input_a = "$%&111001f010100061a024b53535009181c";
        let input_b = "686974207468652062756c6c277320657965";
        xor_str(input_a, input_b);
    }

}
