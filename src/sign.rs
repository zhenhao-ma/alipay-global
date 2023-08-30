use rsa::Hash;
use rsa::PaddingScheme;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::models::{HasPrivateKey};

use super::models::{AlipayClientSecret, RequestEnv, Signable};
use super::sign;

/// Perform a rsa sign for request
fn rsa_sign(content: &str, private_key: &impl HasPrivateKey, hash: Option<Hash>) -> String {
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

/// Perform a rsa verfiy for response
// fn rsa_verify(content: &str, sig: &String, public_key: &impl HasPublicKey, hash: Option<Hash>) {
//     let pk = public_key.get_public_key().unwrap();
//
//     let mut hasher = Sha256::new();
//
//     hasher.input_str(content);
//
//     let mut bytes = vec![0; hasher.output_bytes()];
//     hasher.result(&mut bytes);
//
//     let mut sig_hasher = Sha256::new();
//     sig_hasher.input_str(sig);
//     let mut sig_bytes = vec![0; sig_hasher.output_bytes()];
//     sig_hasher.result(&mut sig_bytes);
//
//     let verify_result = pk.verify(PaddingScheme::PKCS1v15Sign { hash: hash }, &bytes, &sig.as_bytes());
//     verify_result.map(|_| println!("verify success")).map_err(|e| println!("verify: {}", e));
// }

/// Return Alipay Raw Request
fn get_alipay_raw_request(
    method: &str,
    path: &str,
    client_id: &str,
    iso_utc: &str,
    signable: &impl Signable,
) -> String {
    format!(
        "{} {}\n{}.{}.{}",
        method,
        path,
        client_id,
        iso_utc,
        signable.get_value().to_string()
    )
}
/// Sign a request
pub(crate) fn sign(method: &str, utc: chrono::DateTime<chrono::Utc>, secret: &AlipayClientSecret, signable: &impl Signable) -> String {
    let RequestEnv { path, .. } = RequestEnv::from(secret);
    let AlipayClientSecret { client_id, .. } = secret;

    // let utc = chrono::Utc::now();
    let iso_utc = utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, false);

    let content = get_alipay_raw_request(method, &path, &client_id, &iso_utc, signable);
    sign::rsa_sign(&content, secret, Some(Hash::SHA2_256))
}

// fn get_alipay_raw_response(
//     method: &str,
//     path: &str,
//     client_id: &str,
//     iso_utc: &str,
//     verify_content: &String,
// ) -> String {
//     format!(
//         "{} {}\n{}.{}.{}",
//         method,
//         path,
//         client_id,
//         iso_utc,
//         verify_content,
//     )
// }
//
// pub(crate) fn verfiy(method: &str, utc: chrono::DateTime<chrono::Utc>, secret: &AlipayClientSecret, sig: &String, verify_content: &String) {
//     let RequestEnv { path, .. } = RequestEnv::from(secret);
//     let AlipayClientSecret { client_id, .. } = secret;
//
//     // let utc = chrono::Utc::now();
//     let iso_utc = utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
//
//     let content = get_alipay_raw_response(method, &path, &client_id, &iso_utc, verify_content);
//     println!("content: {}", content);
//     let signature: Vec<&str> = sig.split("signature=").collect();
//     let urldecoded_sig = urlencoding::decode(signature[1]).unwrap().into_owned();
//     println!("urldecoded_sig: {}", urldecoded_sig);
//     sign::rsa_verify(&content, &urldecoded_sig, secret, Some(Hash::SHA2_256));
// }