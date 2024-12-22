use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use base64::{engine::general_purpose, Engine as _};
use crate::utils;

pub fn convert_hex_to_base64()-> String{
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let decoded = hex::decode(hex).expect("Decoding failed");
    let base64 = general_purpose::STANDARD.encode(&decoded);
    return base64;
}

pub fn fixed_xor()-> String{
    let string1 = hex::decode("1c0111001f010100061a024b53535009181c").expect("Decoding failed");
    let string2 = hex::decode("686974207468652062756c6c277320657965").expect("Decoding failed");

    let string3: Vec<u8> = string1
        .iter()
        .zip(string2.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();
    return hex::encode(&string3)
}

pub fn single_byte_xor_cipher()->String{
    return get_single_byte_xor_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
}

fn get_single_byte_xor_cipher(text: &str)->String{
    let encoded_string = hex::decode(text).expect("Decoding failed");
    let mut best_matches = HashMap::new();

    for b in 0..u8::MAX {
        let string3: Vec<u8> = encoded_string
            .iter()
            .map(|x| {x ^ b})
            .collect();

        let text = String::from_utf8_lossy(&string3); //str::from_utf8
        let matching_chars = utils::get_text_detection_score(&text);

        //if matching_chars>20 {
            best_matches.insert(b, matching_chars);
        //}
    }
    let max_entry = best_matches.iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap();

    let byte_string: Vec<u8> = encoded_string
        .iter()
        .map(|x| {x ^ max_entry.0})
        .collect();

    return String::from_utf8_lossy(&byte_string).to_string();
    }

pub fn single_byte_xor_cipher_from_file()->String {
    let filename = Path::new("./src/resources/4.txt");
    let mut best_matches = HashMap::new();

    for line in read_to_string(filename).unwrap().lines() {
        let text = get_single_byte_xor_cipher(line);
        let matching_chars = utils::get_text_detection_score(&text);

        if matching_chars>20 {
            best_matches.insert(text, matching_chars);
        }
    }

    let max_entry = best_matches.iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap();

    return max_entry.0.to_string()
}

pub fn repeating_key_xor()->String{
 return apply_xor("Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal","ICE");
}

fn apply_xor(text: &str, xor_key:&str)->String{
    let mut result: Vec<u8> = Vec::from(text.as_bytes());
    let long_key =  Vec::from(xor_key.repeat(result.len() / xor_key.len()) + &xor_key[0..result.len() % xor_key.len()]);

    let string3: Vec<u8> = result
        .iter()
        .zip(long_key.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();


    for b in xor_key.bytes(){
        result = result
            .iter()
            .map(|x| {x ^ b})
            .collect();
    }
    let test = hex::encode(string3);
    println!("{}", test);
    return test.to_string()
}

