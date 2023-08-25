use super::errors::Error;
use super::models::{AlipayClientSecret, PaymentCashier, RequestEnv, Response, Signable};
use super::sign;
use crate::models::PaymentCashierRequest;
use chrono::{self};
use rsa::Hash;
use ureq;
use urlencoding;

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

pub fn parse_response(response_body: String) -> Result<Response, Error> {
    let parsed: Response = serde_json::from_str::<Response>(&response_body).map_err(|e| {
        Error::Unknown(format!(
            "Failed to parse response body into base object: {}",
            e.to_string()
        ))
    })?;
    let e = parsed.get_error();
    match e {
        Some(e) => Err(e),
        None => Ok(parsed),
    }
}

pub fn sign(secret: &AlipayClientSecret, signable: &impl Signable) -> String {
    let RequestEnv { path, .. } = RequestEnv::from(secret);
    let AlipayClientSecret { client_id, .. } = secret;

    let utc = chrono::Utc::now();
    let iso_utc = utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, false);

    let content = get_alipay_raw_request("POST", &path, &client_id, &iso_utc, signable);
    sign::rsa_sign(&content, secret, Some(Hash::SHA2_256))
}

pub fn req_post(
    secret: &AlipayClientSecret,
    payment_cashier: &PaymentCashier,
) -> Result<Response, Error> {
    let request_env = RequestEnv::from(secret);
    let request_url = request_env.get_request_url();
    let payment_cashier_request = PaymentCashierRequest::from(payment_cashier);
    let signed = sign(secret, &payment_cashier_request);

    let resp = ureq::post(&request_url)
        .set("Content-Type", "application/json")
        .set(
            "Signature",
            format!(
                "algorithm=RSA256,keyVersion=1,signature={}",
                urlencoding::encode(&signed)
            )
            .as_str(),
        )
        .set("client-id", &secret.client_id)
        .set(
            "Request-Time",
            &chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
        )
        .send_string(&payment_cashier_request.to_string())?;
    let response_body = resp.into_string().map_err(|e| Error::from(e))?;
    parse_response(response_body)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::models::TerminalType;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_req() {
        let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable");
        let private_key_pem_path =
            env::var("PEM_PATH").expect("Missing PEM_PATH environment variable");
        let secret = AlipayClientSecret {
            client_id: String::from(client_id),
            sandbox: true,
            private_key_pem: None,
            private_key_pem_file: Some(Box::new(PathBuf::from(&private_key_pem_path))),
        };
        let payment_cashier = PaymentCashier {
            payment_request_id: uuid::Uuid::new_v4().to_string(),
            currency: String::from("USD"),
            amount: 100,
            redict_url: String::from("https://google.com"),
            notifiy_url: String::from("https://google.com"),
            reference_order_id: None,
            order_description: String::from("order_description"),
            terminal_type: Some(TerminalType::WEB),
        };
        let r = req_post(&secret, &payment_cashier);
        print!("response: \n{:#?}\n", r);
    }
}
