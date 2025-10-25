use base64::prelude::*;

pub fn hex_to_base64(h: &str) -> String {
    let decoded = hex::decode(h).unwrap();
    BASE64_STANDARD.encode(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_success_case_1() {
        let input: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let output: String =
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".into();
        assert_eq!(hex_to_base64(input), output);
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_invalid_hex() {
        let input: &str = "%$%£%£%";
        hex_to_base64(input);
    }
}
