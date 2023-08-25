use rsa::Hash;
use rsa::PaddingScheme;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::models::HasPrivateKey;

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
pub(crate) fn sign(method: &str, secret: &AlipayClientSecret, signable: &impl Signable) -> String {
    let RequestEnv { path, .. } = RequestEnv::from(secret);
    let AlipayClientSecret { client_id, .. } = secret;

    let utc = chrono::Utc::now();
    let iso_utc = utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, false);

    let content = get_alipay_raw_request(method, &path, &client_id, &iso_utc, signable);
    sign::rsa_sign(&content, secret, Some(Hash::SHA2_256))
}
