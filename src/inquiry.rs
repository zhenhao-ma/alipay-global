use super::errors::Error;
use super::models::{Response, AlipayClientSecret, CashierPaymentInquiry, RequestEnv};
use super::sign::{sign, verify};
use super::response::parse_response;

pub fn cashier_payment(
    secret: &AlipayClientSecret,
    cashier_payment_inquiry: &CashierPaymentInquiry
) -> Result<Response, Error> {
    let utc_now = chrono::Utc::now();
    let request_env = RequestEnv::from(secret);
    let request_url = request_env.get_request_url();
    let signed = sign("POST", utc_now, secret, cashier_payment_inquiry);
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
            &utc_now.to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
        )
        .send_string(&cashier_payment_inquiry.to_string())?;
    let header_signature = resp.header("Signature").unwrap().to_string();
    let response_time = resp.header("Response-Time").unwrap().to_string();
    let client_id = resp.header("Client-Id").unwrap().to_string();
    let response_body = resp.into_string().map_err(|e| Error::from(e))?;
    let verify = verify(
            "POST",
            response_time.as_str(),
            header_signature.as_str(),
            client_id.as_str(),
            response_body.as_str(),
            secret
        )
        .map_err(|_| Error::Fail(String::from("response verification failed")))?;
    parse_response(response_body)
}
