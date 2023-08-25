use super::errors::Error;
use super::models::{AlipayClientSecret, CashierPaymentSimple, RequestEnv, Response, Signable};
use super::response::parse_response;
use super::sign::sign;
use crate::models::CashierPaymentFull;
use chrono::{self};
use rsa::Hash;
use ureq;
use urlencoding;

/// Create A [Cashier Payment](https://global.alipay.com/docs/ac/ams/payment_cashier)
/// Use this API to get the cashier page address. After getting the cashier page address, you can redirect the user to the cashier page to make a payment.
pub fn cashier_payment(
    secret: &AlipayClientSecret,
    cashier_payment: &CashierPaymentSimple,
) -> Result<Response, Error> {
    let request_env = RequestEnv::from(secret);
    let request_url = request_env.get_request_url();
    let payment_cashier_request = CashierPaymentFull::from(cashier_payment);
    let signed = sign("POST", secret, &payment_cashier_request);

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
    use std::path::PathBuf;

    use crate::models::TerminalType;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_req() {
        let client_id = std::env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable");
        let private_key_pem_path =
            std::env::var("PEM_PATH").expect("Missing PEM_PATH environment variable");
        let secret = AlipayClientSecret {
            client_id: String::from(client_id),
            sandbox: true,
            private_key_pem: None,
            private_key_pem_file: Some(Box::new(PathBuf::from(&private_key_pem_path))),
        };
        let payment_cashier = CashierPaymentSimple {
            payment_request_id: uuid::Uuid::new_v4().to_string(),
            currency: String::from("USD"),
            amount: 100,
            redict_url: String::from("https://google.com"),
            notifiy_url: String::from("https://google.com"),
            reference_order_id: None,
            order_description: String::from("order_description"),
            terminal_type: Some(TerminalType::WEB),
        };
        let r = cashier_payment(&secret, &payment_cashier);
        print!("response: \n{:#?}\n", r);
    }
}