use super::errors::Error;
use chrono::{DateTime, Utc};
use rsa::{errors::Error as RSAError, RSAPrivateKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::string::ToString;
use strum_macros::Display;

/// Alipay Client Info and Secret
pub struct AlipayClientSecret {
    pub client_id: String,
    pub sandbox: bool,
    pub private_key_pem: Option<String>,
    pub private_key_pem_file: Option<Box<PathBuf>>,
}

impl HasPrivateKey for AlipayClientSecret {
    fn get_private_key(&self) -> Result<RSAPrivateKey, RSAError> {
        let der_bytes: Vec<u8>;
        if (self.private_key_pem_file.is_some()) {
            let s = read_to_string(self.private_key_pem_file.clone().unwrap().as_path()).unwrap();
            der_bytes = base64::decode(s).expect("failed to decode base64 content");
        } else {
            der_bytes = base64::decode(&self.private_key_pem.clone().unwrap())
                .expect("failed to decode base64 content");
        }

        // get private obj
        return RSAPrivateKey::from_pkcs1(&der_bytes);
    }
}

/// Minimum Information to initialize a payment cashier
#[derive(Serialize)]
pub struct PaymentCashier {
    pub payment_request_id: String,
    pub currency: String,
    pub amount: i32,
    pub redict_url: String,
    pub notifiy_url: String,
    pub order_description: String,
    pub reference_order_id: Option<String>,
    pub terminal_type: Option<TerminalType>,
}

/// A Trait contains all data for alipay signing
pub trait Signable {
    fn get_value(&self) -> Value;
}

/// A Trait contains private key data
pub trait HasPrivateKey {
    fn get_private_key(&self) -> Result<RSAPrivateKey, RSAError>;
}

/// Payment Cashier Request Object
/// see: https://global.alipay.com/docs/ac/ams/payment_cashier
/// skip attributes
/// - paymentFactor
/// - paymentExpiryTime
/// - userRegion
/// - creditPayPlan
/// - appId
/// - merchantRegion
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentCashierRequest {
    /// Represents the payment product that is being used, which is stipulated in the contract. For Cashier Payment, the value is fixed as CASHIER_PAYMENT.
    pub product_code: String,
    /// The unique ID assigned by a merchant to identify a payment request. Alipay uses this field for idempotence control.
    /// More information about this field:
    /// This field is an API idempotency field. For payment requests that are initiated with the same value of paymentRequestId and reach a final status of S or F, the same result is to be returned for the request.
    /// Maximum length: 64 characters
    pub payment_request_id: String,
    pub order: Order,
    pub payment_amount: Amount,
    pub payment_method: PaymentMethod,
    pub payment_redirect_url: String,
    pub payment_notify_url: String,
    pub settlement_strategy: SettlementStrategy,
    pub env: Env,
}

impl PaymentCashierRequest {
    pub fn to_string(&self) -> String {
        serde_json::to_value(self).unwrap().to_string()
    }
}

impl From<&PaymentCashier> for PaymentCashierRequest {
    fn from(value: &PaymentCashier) -> Self {
        let PaymentCashier {
            payment_request_id,
            redict_url,
            notifiy_url,
            ..
        } = value;
        let order = Order::from(value);
        let payment_amount = Amount::from(value);
        let payment_method = PaymentMethod::from(value);
        let settlement_strategy = SettlementStrategy::from(value);
        let env = Env::from(value);
        Self {
            product_code: String::from("CASHIER_PAYMENT"),
            payment_request_id: payment_request_id.clone(),
            order,
            payment_amount,
            payment_method,
            payment_redirect_url: redict_url.clone(),
            payment_notify_url: notifiy_url.clone(),
            settlement_strategy,
            env,
        }
    }
}

impl Signable for PaymentCashierRequest {
    fn get_value(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}

pub struct RequestEnv {
    pub path: String,
    pub domain: String,
}
impl From<&AlipayClientSecret> for RequestEnv {
    fn from(value: &AlipayClientSecret) -> Self {
        if value.sandbox {
            Self {
                path: String::from("/ams/sandbox/api/v1/payments/pay"),
                domain: String::from("https://open-global.alipay.com"),
            }
        } else {
            Self {
                path: String::from("/ams/api/v1/payments/pay"),
                domain: String::from("https://open-global.alipay.com"),
            }
        }
    }
}

impl RequestEnv {
    pub fn get_request_url(&self) -> String {
        self.domain.clone() + &self.path
    }
}

/// Information about the environment where the order is placed, such as the device information.
/// skip attributes
/// - osType
/// - browserInfo
/// - colorDepth
/// - screenHeight
/// - screenWidth
/// - timeZoneOffset
/// - deviceBrand
/// - deviceModel
/// - deviceTokenId
/// - clientIp
/// - deviceLanguage
/// - deviceId
/// - extendInfo
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    /// Terminal type of which the merchant service applies to. Valid values are:
    /// WEB: The client-side terminal type is a website, which is opened via a PC browser.
    /// WAP: The client-side terminal type is an H5 page, which is opened via a mobile browser.
    /// APP: The client-side terminal type is a mobile application.
    /// MINI_APP: The terminal type of the merchant side is a mini program on the mobile phone.  
    terminal_type: TerminalType,
}

impl From<&PaymentCashier> for Env {
    fn from(value: &PaymentCashier) -> Self {
        let PaymentCashier { terminal_type, .. } = value;
        let tt = terminal_type.clone().unwrap_or(TerminalType::WEB);
        Self { terminal_type: tt }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub enum TerminalType {
    WEB,
    WAP,
    APP,
    MINI_APP,
}

/// The order amount of the merchant that directly provides services or goods to the customer. This field is used for user consumption records display or payment results page.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    /// The transaction currency that is specified in the contract. A 3-letter currency code that follows the ISO 4217 standard.
    /// More information about this field:
    /// Maximum length: 3 characters
    pub currency: String,
    /// The amount to charge as a positive integer in the smallest currency unit. (That is, 100 cents to charge $1.00, or 100 to charge JPY 100, a 0-decimal currency).
    /// Note: For details about the smallest currency unit, see Smallest unit of the currency.
    /// More information about this field:
    /// Value range: 1 - unlimited
    ///
    /// value can be i32 or String when you are sending request
    /// HOWEVER, value can only be String when deserializing from response body
    value: String,
}
impl Amount {
    pub fn value(&self) -> u32 {
        self.value.parse().unwrap()
    }
}
impl From<&PaymentCashier> for Amount {
    fn from(value: &PaymentCashier) -> Self {
        let PaymentCashier {
            currency, amount, ..
        } = value;
        Self {
            value: amount.to_string().clone(),
            currency: currency.clone(),
        }
    }
}

/// The payment method that is used to collect the payment by the merchant or acquirer.
/// skip attributes
/// - paymentMethodId
/// - paymentMethodMetaData
/// - customerId
/// - extendInfo
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    /// The payment method type that is included in payment method options. By specifying the value of this parameter, you can receive the cashier URL of the specified payment method returned by Alipay. See Payment methods to check the valid values.
    /// More information about this field:
    /// Maximum length: 64 characters
    pub payment_method_type: String,
}

impl From<&PaymentCashier> for PaymentMethod {
    fn from(value: &PaymentCashier) -> Self {
        Self {
            payment_method_type: String::from("ALIPAY_CN"),
        }
    }
}

/// The order information, such as buyer, merchant, goods, amount, shipping information, and purchase environment. This field is used for different purposes:
/// During the payment process, this field is mainly used by Alipay for risk control or anti-money laundering.
/// After the payment is completed, this field is used for recording and reporting purposes such as purchase tracking and regulatory reporting.
/// skip attributes
/// - goods
/// - buyer
/// - merchant
/// - extendInfo
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// The order amount of the merchant that directly provides services or goods to the customer. This field is used for user consumption records display or payment results page.
    pub order_amount: Amount,
    pub order_description: String,
    pub reference_order_id: String,
}

impl From<&PaymentCashier> for Order {
    fn from(value: &PaymentCashier) -> Self {
        let PaymentCashier {
            reference_order_id,
            order_description,
            ..
        } = value;
        let roi = reference_order_id.clone().unwrap_or(String::from(""));
        let od = order_description.clone();
        let order_amount = Amount::from(value);
        Self {
            order_amount,
            order_description: od,
            reference_order_id: roi,
        }
    }
}

/// The settlement strategy for the payment request.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementStrategy {
    /// The ISO currency code of the currency that the merchant wants to be settled against. The field is required if the merchant signed up for multiple currencies to settle.
    settlement_currency: String,
}

impl From<&PaymentCashier> for SettlementStrategy {
    fn from(value: &PaymentCashier) -> Self {
        let PaymentCashier { currency, .. } = value;

        Self {
            settlement_currency: currency.clone(),
        }
    }
}

///
/// skip attributes
///
///
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    result: ResponseResult,
    payment_request_id: Option<String>,
    payment_id: Option<String>,
    payment_amount: Option<Amount>,
    payment_data: Option<String>,
    payment_create_time: Option<DateTime<Utc>>,
    psp_customer_info: Option<PspCustomerInfo>,
    order_code_form: Option<OrderCodeForm>,
    // this field is actually option, do NOT trust the Alipay API doc
    gross_settlement_amount: Option<Amount>,
    // this field is actually option, do NOT trust the Alipay API doc
    settlement_quote: Option<SettlementQuote>,
    app_identifier: Option<String>,
    applink_url: Option<String>,
    normal_url: Option<String>,
    scheme_url: Option<String>,
    payment_result_info: Option<PaymentResultInfo>,
}

impl Response {
    pub fn result(&self) -> &ResponseResult {
        &self.result
    }
    pub fn payment_request_id(&self) -> &Option<String> {
        &self.payment_request_id
    }
    pub fn payment_id(&self) -> &Option<String> {
        &self.payment_id
    }
    pub fn is_success(&self) -> bool {
        self.result.result_status == ResultStatus::S
    }
    pub fn is_processing(&self) -> bool {
        self.result.result_code == ResultCode::PAYMENT_IN_PROCESS
    }
    pub fn get_error(&self) -> Option<Error> {
        self.result.get_error()
    }
}

/// The result of the API call.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseResult {
    /// Result code. The result code that might be returned are listed in the Result/Error codes table on this page.
    /// More information about this field:
    /// Maximum length: 64 characters
    result_code: ResultCode,
    result_status: ResultStatus,
    result_message: String,
}

impl ResponseResult {
    pub fn get_error(&self) -> Option<Error> {
        if self.result_code == ResultCode::PAYMENT_IN_PROCESS {
            // payment in process will have a result status code as U
            return None;
        }
        match self.result_status {
            ResultStatus::S => None,
            ResultStatus::F => Some(Error::Fail(self.result_code.to_string())),
            ResultStatus::U => Some(Error::Unknown(self.result_code.to_string())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Display)]
pub enum ResultCode {
    SUCCESS,
    ACCESS_DENIED,
    INVALID_API,
    CURRENCY_NOT_SUPPORT,
    EXPIRED_CODE,
    FRAUD_REJECT,
    INVALID_ACCESS_TOKEN,
    INVALID_CONTRACT,
    INVALID_MERCHANT_STATUS,
    INVALID_PAYMENT_CODE,
    INVALID_PAYMENT_METHOD_META_DATA,
    KEY_NOT_FOUND,
    MERCHANT_KYB_NOT_QUALIFIED,
    MERCHANT_NOT_REGISTERED,
    NO_INTERFACE_DEF,
    NO_PAY_OPTIONS,
    ORDER_IS_CANCELED,
    ORDER_IS_CLOSED,
    PARAM_ILLEGAL,
    PAYMENT_AMOUNT_EXCEED_LIMIT,
    PAYMENT_COUNT_EXCEED_LIMIT,
    PAYMENT_NOT_QUALIFIED,
    PROCESS_FAIL,
    REPEAT_REQ_INCONSISTENT,
    RISK_REJECT,
    SETTLE_CONTRACT_NOT_MATCH,
    SYSTEM_ERROR,
    USER_AMOUNT_EXCEED_LIMIT,
    USER_BALANCE_NOT_ENOUGH,
    USER_KYC_NOT_QUALIFIED,
    PAYMENT_IN_PROCESS,
    REQUEST_TRAFFIC_EXCEED_LIMIT,
    UNKNOWN_EXCEPTION,
    USER_NOT_EXIST,
    ORDER_NOT_EXIST,
    ORDER_STATUS_INVALID,
    USER_PAYMENT_VERIFICATION_FAILED,
    USER_STATUS_ABNORMAL,
    VERIFY_TIMES_EXCEED_LIMIT,
    VERIFY_UNMATCHED,
    AUTHENTICATION_REQUIRED,
    SELECTED_CARD_BRAND_NOT_AVAILABLE,
    PAYMENT_PROHIBITED,
}

/// Result status. Valid values are:
/// S: Indicates that the API call succeeds.
/// F: Indicates that the API call fails.
/// U: Indicates that the API call might be successful, in process, or failed. For more details, see Result process logic.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultStatus {
    S,
    F,
    U,
}
/// Information about the order code.
/// This parameter is returned when the payment method supports providing the related information.  
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCodeForm {
    expire_time: chrono::DateTime<chrono::Utc>,
    code_details: Vec<CodeDetail>,
    extend_info: Option<String>,
}

/// Details about the code.
/// More information about this field:
/// Maximum size: 4 elements
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CodeDetail {
    code_value: String,
    display_type: DisplayType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PspCustomerInfo {
    psp_name: Option<String>,
    psp_customer_id: Option<String>,
    display_customer_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum DisplayType {
    TEXT,
    MIDDLEIMAGE,
    SMALLIMAGE,
    BIGIMAGE,
}

/// The exchange rate between the settlement currency and transaction currency. This field is returned when grossSettlementAmount is returned.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettlementQuote {
    guaranteed: Option<bool>,
    quote_id: Option<String>,
    quote_currency_pair: String,
    quote_price: i32,
    quote_start_time: Option<DateTime<Utc>>,
    quote_expiry_time: Option<DateTime<Utc>>,
}

/// The payment result information.
/// This parameter may be returned when the value of paymentMethodType is CARD.  
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResultInfo {
    avs_result_raw: Option<String>,
    cvv_result_raw: Option<String>,
    network_transaction_id: Option<String>,
}
