use chrono::{DateTime, Utc};
use urlencoding;

use super::errors::Error;
use super::models::{
    Response, ResponseResult, ResultCode, ResultStatus,
    AlipayClientSecret, CashierPaymentInquiry, RequestEnv,
    NotifyPayment, WebhookData, WebhookResponse, WebhookResponseInput,
    WebhookResponseResult
};
use super::sign::{sign, verify};
use super::response::parse_response;

pub fn cashier_payment(
    secret: &AlipayClientSecret,
    webhook_data: WebhookData,
) -> Result<Response, Error> {
    let verify = verify(
        Some(webhook_data.path),
        webhook_data.method.as_str(),
        webhook_data.request_time.as_str(),
        webhook_data.header_signature.as_str(),
        webhook_data.client_id.as_str(),
        webhook_data.request_body.as_str(),
        secret
    ).map_err(|_| Error::Fail(String::from("webhook verification failed")))?;
    serde_json::from_str::<Response>(&webhook_data.request_body).map_err(|e| {
        Error::Unknown(format!(
            "Failed to parse response body into base object: {}",
            e.to_string()
        ))
    })
}

pub fn success_response(
    secret: &AlipayClientSecret,
    webhook_response_in: WebhookResponseInput,
) -> Result<WebhookResponse, Error> {
    let utc_now = Utc::now();
    let response_result = ResponseResult {
        result_code: ResultCode::SUCCESS,
        result_status: ResultStatus::S,
        result_message: String::from("Success")
    };
    let response_result_content = WebhookResponseResult {
        result: response_result,
    };
    let signed = sign(
        webhook_response_in.method.as_str(),
        Some(webhook_response_in.path),
        Some(webhook_response_in.client_id.to_owned()),
        utc_now,
        secret,
        &response_result_content
    );
    let response = WebhookResponse {
        full_signature: format!(
            "algorithm=RSA256,keyVersion=1,signature={}",
            urlencoding::encode(&signed)
        ),
        client_id: webhook_response_in.client_id,
        response_time: utc_now.to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
        body: serde_json::to_value(response_result_content).unwrap().to_string(),
    };
    Ok(response)
}

pub fn failed_response(
    secret: &AlipayClientSecret,
    webhook_response_in: WebhookResponseInput,
) -> Result<WebhookResponse, Error> {
    let utc_now = Utc::now();
    let response_result = ResponseResult {
        result_code: ResultCode::PARAM_ILLEGAL,
        result_status: ResultStatus::F,
        result_message: String::from("The required parameters are not passed, or illegal parameters exist. For example, a non-numeric input, an invalid date, or the length and type of the parameter are wrong.")
    };
    let response_result_content = WebhookResponseResult {
        result: response_result,
    };
    let signed = sign(
        webhook_response_in.method.as_str(),
        Some(webhook_response_in.path),
        Some(webhook_response_in.client_id.to_owned()),
        utc_now,
        secret,
        &response_result_content
    );
    let response = WebhookResponse {
        full_signature: format!(
            "algorithm=RSA256,keyVersion=1,signature={}",
            urlencoding::encode(&signed)
        ),
        client_id: webhook_response_in.client_id,
        response_time: utc_now.to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
        body: serde_json::to_value(response_result_content).unwrap().to_string(),
    };
    Ok(response)
}
