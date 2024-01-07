#[cfg(test)]
mod tests {
    use crate::{basics, utils};

    #[test]
    fn convert_hex_to_base64() {
        assert_eq!(basics::convert_hex_to_base64(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn fixed_xor() {
        let expected_result = "746865206b696420646f6e277420706c6179";
        assert_eq!(basics::fixed_xor(), expected_result);
    }

    #[test]
    fn single_byte_xor_cipher(){
        assert_eq!(basics::single_byte_xor_cipher(), "Cooking MC's like a pound of bacon");
    }
    #[test]
    fn single_byte_xor_cipher_from_file(){
        //assert_eq!(basics::single_byte_xor_cipher_from_file(), "Now that the party is jumping\n");
    }

    #[test]
    fn repeating_key_xor(){
        assert_eq!(basics::repeating_key_xor(), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }

    #[test]
    fn hamming_distance_test(){
        let string1 = "this is a test";
        let string2 = "wokka wokka!!!";

        let hamming_distance = utils::get_hamming_distance(string1.as_ref(), string2.as_ref());
        assert_eq!(hamming_distance, 37);
    }
}