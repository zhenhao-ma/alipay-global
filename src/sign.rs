extern crate crypto;
extern crate rsa;

use rsa::Hash;
use rsa::{PaddingScheme, RSAPrivateKey};

use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub enum HashType {
    Sha1,
    Sha256,
}

pub fn rsa_sign(content: &str, private_key: &str, hash_type: HashType) -> String {
    // format private key
    let der_encoded = private_key;
    let der_bytes = base64::decode(der_encoded).expect("failed to decode base64 content");
    // get private obj
    let private_key = RSAPrivateKey::from_pkcs1(&der_bytes).expect("failed to parse key");

    // create sha256 obj
    let mut hasher = Sha256::new();
    
    // input content
    hasher.input_str(content);

    // save result to bytes
    let mut bytes = vec![0; hasher.output_bytes()];
    hasher.result(&mut bytes);

    // sign the content
    let hash;
    match hash_type {
        HashType::Sha1 => hash = Hash::SHA1,
        HashType::Sha256 => hash = Hash::SHA2_256,
    }
    let sign_result = private_key.sign(
        PaddingScheme::PKCS1v15Sign {
            hash: Option::from(hash),
        },
        &bytes,
    );
    // convert result to base64
    let vec = sign_result.expect("create sign error for base64");

    base64::encode(vec)
}