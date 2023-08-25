extern crate crypto;
extern crate rsa;

use rsa::Hash;
use rsa::PaddingScheme;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::models::HasPrivateKey;

pub fn rsa_sign(content: &str, private_key: &impl HasPrivateKey, hash: Option<Hash>) -> String {
    // get private obj
    let pk = private_key.get_private_key().unwrap();

    // create sha256 obj
    let mut hasher = Sha256::new();

    // input content
    hasher.input_str(content);

    // save result to bytes
    let mut bytes = vec![0; hasher.output_bytes()];
    hasher.result(&mut bytes);

    // sign the content
    let sign_result = pk.sign(PaddingScheme::PKCS1v15Sign { hash }, &bytes);
    // convert result to base64
    let vec = sign_result.expect("create sign error for base64");

    base64::encode(vec)
}
