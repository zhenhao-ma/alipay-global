use super::errors::Error;
use super::models::{Response, AlipayClientSecret, CashierPaymentRefundSimple, CashierPaymentRefundFull, RequestEnv};
use super::sign::sign;
use super::response::parse_response;

pub fn cashier_payment(
    secret: &AlipayClientSecret,
    cashier_payment_refund: &CashierPaymentRefundSimple
) -> Result<Response, Error> {
    let utc_now = chrono::Utc::now();
    let request_env = RequestEnv::from(secret);
    let request_url = request_env.get_request_url();
    let payment_cashier_refund_request = CashierPaymentRefundFull::from(cashier_payment_refund);
    let signed = sign("POST", utc_now, secret, &payment_cashier_refund_request);
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
        .send_string(&payment_cashier_refund_request.to_string())?;
    let response_body = resp.into_string().map_err(|e| Error::from(e))?;
    println!("response_body: {}", response_body);
    parse_response(response_body)
}