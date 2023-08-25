use super::errors::Error;
use super::models::Response;

/// Prase Alipay Response
pub(crate) fn parse_response(response_body: String) -> Result<Response, Error> {
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
