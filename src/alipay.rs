mod sign;

use chrono::{self, DateTime, Utc};
use serde::Serialize;
use serde_json::{self, Number};
use std::collections::HashMap;
use ureq;
use url;
use urlencoding;
use uuid::Uuid;

// use crate::services::alipay_gb::model::{AlipayPaymentPublic};

#[derive(Serialize)]
pub struct OrderAmount {
    currency: String,
    value: Number,
}

#[derive(Debug)]
pub struct Info {
    pub payment_request_id: String,
    pub timestamp: i64,
    pub datetime: chrono::DateTime<chrono::Utc>,
    pub sign: String,
    pub request_body: serde_json::Value,
}

#[derive(Debug)]
pub struct SendInfo {
    pub send_request_time: chrono::DateTime<chrono::Utc>,
}

pub struct AlipayClient {
    client_id: String,

}

pub struct AlipaySign {
    order_amount: Number,
    currency: String,
    notify_url: Option<String>,
    redirect_url: Option<String>,
    payment_request_id: Option<String>,
    datetime: Option<DateTime<Utc>>,
    sandbox: Option<bool>
}



pub fn alipay_pay(
    alipay_sign: AlipaySign
    // request_url: &String,
    // order_amount: OrderAmount,
    // notify_url: Option<&str>,
    // redirect_url: Option<&str>,
    // payment_request_id: Option<&str>,
    // datetime: Option<DateTime<Utc>>,
) -> Info {
    let request_path = String::from("/ams/sandbox/api/v1/payments/pay");
    let request_url = String::from("https://open-global.alipay.com") + &request_path;
    let private_key = "MIIEoQIBAAKCAQBCpIE9yzFF6ejTTeLKqQIIDJTtCMgBnQqy39Z7ombLdp92zrXmvvYIh3NbV+ISAIl6/NU0WmqbYU9EwPcxZkuevdDrhoAFUZoXOSqrqZzj5IZnsaUrUa/Mw/c5edv/Zp7WDetK5oOrhzrdo8ZabmtIB7h37hf0zfiuB107J0ShqTaOMbN4SIwEmtxCp4qTgoi8Z7qXR9HZOzzwnTQxhh6WsYfq7KbKmwlI/Yb6+Kno3t1B5tKXwqRuelhy1p8RcLMyhRK2AD5iENccauiuKYmQq6JE2FAS5tr28SvM6jlfR2a4POIh50yuFwnRSVbyfS8VvNL13WHEq2Gej10TL30pAgMBAAECggEAFB1dGQ6sh6KrcKPwkSTkBRPvG4BsBfilkwn2zghdqIncZdrMkqIO1tIzYl2rUa2x0Vpg69VimhWL/H+V3OY4auh2F7DYEULpFJtfosKmJS8D1maLKQEV4+M+Sq3aVkNeK9O1sjzTf0Fo5h8Zro/nd61E44YM2woURkrYvBMFJxoCDpLCV3F1Y6ek41sdPPL4hJ0sbIQDlFNS+QB6ChBqYSe+paVNSK7tfS3+ISyCVxfjdvFbaGKPBzlNhAkNtRc/fez+6qP9MjSANOq2DjCYn0NggUVyhWP7jBatMsTNNAGyAhd+TLWE5N7/9f3ETM9ruQ3QKMKKj/mmwcgL4UjlAQKBgQCEvreAqfpJmYaDC7fh7aHm5+ezSVvssulIUIaY7WJj0EBr7iNgeZAdA/kbbGHrR9dLw86pczBWIB0k6kLe/feWP3g71p02xBdKMz0o9hrzO0fDsrVZejA9wBg+VDusX9xMHFjrY5mxWpyO+WSGkbp1Ij2A8AOxlR34ExyHJjW4CQKBgQCAhVl+8HlytxySiI88P1MbxbLjk4rOHzq1WV51VwdRGURl9Qw5GnBYSuzmq1mZ8cUa/MvJgcc0zQHjUP+BnhRUFDAEV290tpS3WrEHrRmQcmfLM5+2C53ET2u4xFoekY64JrJhC0HFLK45Fiw/msGOAgYhpvqBFYy61sWaLrWkIQKBgBDfVN+ruz5jny9E7AhxdeStkUu+hUqeqvwgEBucAKeDLs0JJcH7cY/ek2ki64dGSF0+9COhmoE377xjckB5s2CLjLK+Ypk1b2hk/t8X+PD1lfeP3XEUENGoeuxhNHyCarPZ3ot8y2o5hDDADkD0gOimg37CAxYtR/PiKfwbpTRRAoGAdyaqBUS+47qd88BFQy7WRx4vrktAWb6cODsllpXbw7UwM3JPJbW8SC8WmPhtNoruGQobiLMX03zo7i1O5IWBBT4+EhhZzZPkJmUfUSWLWN6oGby/qg/08WZMUV1Ay6xY66N8pvm/vSSiVWyYE1PPdG+t7Y8YGYq4ERRC2KHLZuECgYAS/+A2LFMgupSWc2jrVkeXz1TzsGYjDrTBzVi0pUeV+Qez/yGKQtxNPf18uRrX+wFVlgCPWkb+7WvUtLYqRK5hdOShXOiWi75CpXDeicp7Q8WY4Rk6t4j1Zo45L2gPu5sbJ8GRfQMFjNSHCosju5LcJwe3OFRLSdb0WwIDkw2KDA==";

    let client_id = String::from("SANDBOX_5Y925A2Z803200040");
    let default_pri = Uuid::new_v4().to_string();
    let pri = payment_request_id.unwrap_or(&default_pri);

    let request_body = ureq::json!({
        "productCode": "CASHIER_PAYMENT",
        "paymentRequestId": pri,
        "order": {
            "orderAmount": order_amount,
            "referenceOrderId": "channel_id",
            "orderDescription": "description"
        },
        "paymentAmount": order_amount,
        "paymentMethod": {
            "paymentMethodType": "ALIPAY_CN"
        },
        "paymentRedirectUrl": redirect_url.unwrap_or("https://www.baidu.com"),
        "paymentNotifyUrl": notify_url.unwrap_or("https://www.baidu.com"),
        "settlementStrategy": {
            "settlementCurrency": "USD"
        },
        "env": {
            "terminalType": "WEB"
        }
    });
    let _d = datetime.unwrap_or(chrono::Utc::now());
    let iso_utc = _d.to_rfc3339_opts(chrono::SecondsFormat::Secs, false);

    let ts_milliseconds = _d.timestamp() * 1000;

    let content = format!(
        "POST {}\n{}.{}.{}",
        request_path,
        client_id,
        iso_utc,
        request_body.to_string()
    );
    let sign = sign::rsa_sign(&content, private_key, sign::HashType::Sha256);

    Info {
        request_body,
        datetime: _d,
        payment_request_id: pri.to_owned(),
        sign,
        timestamp: ts_milliseconds,
    }
}

pub fn alipay_send(info: &Info) -> Result<String, ureq::Error> {
    let request_path = String::from("/ams/sandbox/api/v1/payments/pay");
    let request_url = String::from("https://open-global.alipay.com") + &request_path;
    let resp = ureq::post(&request_url)
        .set("Content-Type", "application/json")
        .set(
            "Signature",
            format!(
                "algorithm=RSA256,keyVersion=1,signature={}",
                urlencoding::encode(&info.sign)
            )
            .as_str(),
        )
        .set("client-id", "SANDBOX_5Y925A2Z803200040")
        .set(
            "Request-Time",
            &chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
        )
        .send_string(&info.request_body.to_string())?;
    println!("status_text: {}", resp.status_text());
    println!("Status: {}", resp.status());
    let response_body = resp.into_string()?;
    println!("Response body:\n{}", response_body);
    Ok(response_body)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let oa = OrderAmount {
            currency: String::from("USD"),
            value: String::from("123"),
        };
        let r = alipay_pay(oa, None, None, None, None);
        let pretty = format!("{:#}", r.request_body);
        print!("Request Time: {:#?}\n", r.timestamp);
        print!("Request Payment ID: {:#?}\n", r.payment_request_id);
        print!("Pretty Format\n{}\n", pretty);
        let res = alipay_send(&r).unwrap();
        print!("r\n{}", res)
    }
}
