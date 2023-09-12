use std::{
    borrow::BorrowMut,
    io::{Error, Result, ErrorKind},
};
use base64::Engine;
use rsa::{
    pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePublicKey, Hash, PaddingScheme, PublicKey,
    RsaPrivateKey, RsaPublicKey,
};

use sha2::{Digest, Sha256};

use crate::models::{HasPrivateKey, HasPublicKey};

use super::models::{AlipayClientSecret, RequestEnv, Signable};

/// Perform a rsa sign for request
fn rsa_sign(content: &str, private_key: &impl HasPrivateKey, hash: Option<Hash>) -> String {
    let digest = Sha256::digest(content.as_bytes());

    let pk = private_key.get_private_key().unwrap();

    let signature_byte = pk.sign(
        PaddingScheme::new_pkcs1v15_sign(hash),
        digest.as_slice(),
    ).unwrap();
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(signature_byte)
}

// Perform a rsa verfiy for response
fn rsa_verify(content: &str, signature: &str, public_key: &impl HasPublicKey, hash: Option<Hash>) -> Result<()> {
    let pbk = public_key.get_public_key().unwrap();

    let mut hashed = Sha256::new();
    hashed.update(content.as_bytes());
    if let Ok(decode_signature) = base64::decode(signature) {
        match pbk.verify(
            PaddingScheme::new_pkcs1v15_sign(hash),
            &hashed.finalize(),
            &decode_signature,
        ) {
            Ok(()) => Ok(()),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    } else {
        Err(Error::new(ErrorKind::Other, "base64 decode signature failed"))
    }
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
pub(crate) fn sign(
    method: &str,
    sign_path: Option<String>,
    sign_client_id: Option<String>,
    utc: chrono::DateTime<chrono::Utc>,
    secret: &AlipayClientSecret,
    signable: &impl Signable
) -> String {
    let RequestEnv { path, .. } = RequestEnv::from(secret);
    let AlipayClientSecret { client_id, .. } = secret;

    // let utc = chrono::Utc::now();
    let iso_utc = utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, false);

    let content = get_alipay_raw_request(
        method,
        sign_path.unwrap_or(path).as_str(),
        sign_client_id.unwrap_or(client_id.to_owned()).as_str(),
        &iso_utc,
        signable
    );
    rsa_sign(&content, secret, Some(Hash::SHA2_256))
}

fn get_alipay_raw_response(
    method: &str,
    path: &str,
    client_id: &str,
    iso_utc: &str,
    verify_content: &str,
) -> String {
    format!(
        "{} {}\n{}.{}.{}",
        method,
        path,
        client_id,
        iso_utc,
        verify_content,
    )
}

pub(crate) fn verify(
    verify_path: Option<String>,
    method: &str,
    response_time: &str,
    header_signature: &str,
    client_id: &str,
    response_body: &str,
    secret: &AlipayClientSecret
) -> Result<()> {
    let RequestEnv { path, .. } = RequestEnv::from(secret);

    let content = get_alipay_raw_response(
        method,
        verify_path.unwrap_or(path).as_str(),
        client_id,
        response_time,
        response_body
    );
    println!("content: {}", content);
    let signature: Vec<&str> = header_signature.split("signature=").collect();
    let urldecoded_sig = urlencoding::decode(signature[1]).unwrap().into_owned();
    println!("urldecoded_sig: {}", urldecoded_sig);
    rsa_verify(&content, &urldecoded_sig, secret, Some(Hash::SHA2_256))
}
