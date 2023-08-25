//! use [Alipay Global](https://global.alipay.com) without any pain.
//!
//! # Example
//! ```
//! use alipay_global::pay::cashier_payment;
//! use alipay_global::models::*;
//! use std::path::PathBuf;
//!
//! // load client id and private pem key from environment for test purpose
//! let client_id = std::env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable");
//! let private_key_pem_path = std::env::var("PEM_PATH").expect("Missing PEM_PATH environment variable");
//!
//! // Client secret object contains all the information regarding your Alipay Global Account
//! let secret = AlipayClientSecret {client_id: String::from(client_id),
//!     sandbox: true,
//!     private_key_pem: None,
//!     private_key_pem_file: Some(Box::new(PathBuf::from(&private_key_pem_path))),
//! };
//!
//! // CashierPayment Object contains order info
//! let p = CashierPaymentSimple {
//!     payment_request_id: uuid::Uuid::new_v4().to_string(),
//!     currency: String::from("USD"),
//!     amount: 100,
//!     redict_url: String::from("https://google.com"),
//!     notifiy_url: String::from("https://google.com"),
//!     reference_order_id: None,
//!     order_description: String::from("order_description"),
//!     terminal_type: Some(TerminalType::WEB),
//! };
//!
//! // Call the API
//! let r = cashier_payment(&secret, &p);
//! ```
extern crate crypto;
extern crate rsa;
pub use rsa::Hash;
pub mod errors;
pub mod models;
pub mod pay;
mod response;
mod sign;
