use serde::Serialize;
use std::path::Path;

/// Alipay Client Info and Secret
pub struct AlipayClient {
    client_id: String,
    sandbox: bool,
    private_key: Option<String>,
    private_key_der_file: Option<Box<Path>>,
    private_key_per_file: Option<Box<Path>>
}

/// Minimum Information to initialize a payment cashier
#[derive(Serialize)]
pub struct PaymentCashier {
    pub payment_request_id: String,
    pub currency: String,
    pub amount: i32,
    pub redict_url: String,
    pub notifiy_url: String,
    pub reference_order_id: Option<String>,
    pub order_description: Option<String>,
    pub terminal_type: Option<TerminalType>,
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
    pub payment_amount: OrderAmount,
    pub payment_method: PaymentMethod,
    pub payment_redirect_url: String,
    pub payment_notify_url: String,
    pub settlement_strategy: SettlementStrategy,
}

impl From<PaymentCashier> for PaymentCashierRequest {
    fn from(value: PaymentCashier) -> Self {
        let PaymentCashier {
            payment_request_id,
            redict_url,
            notifiy_url,
            ..
        } = value;
        let order = Order::from(&value);
        let payment_amount = OrderAmount::from(&value);
        let payment_method = PaymentMethod::from(&value);
        let settlement_strategy = SettlementStrategy::from(&value);
        Self {
            product_code: String::from("CASHIER_PAYMENT"),
            payment_request_id,
            order,
            payment_amount,
            payment_method,
            payment_redirect_url: redict_url,
            payment_notify_url: notifiy_url,
            settlement_strategy,
        }
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
        let tt = terminal_type.unwrap_or(TerminalType::WEB);
        Self { terminal_type: tt }
    }
}

#[derive(Serialize)]
pub enum TerminalType {
    WEB,
    WAP,
    APP,
    MINI_APP,
}

/// The order amount of the merchant that directly provides services or goods to the customer. This field is used for user consumption records display or payment results page.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmount {
    /// The transaction currency that is specified in the contract. A 3-letter currency code that follows the ISO 4217 standard.
    /// More information about this field:
    /// Maximum length: 3 characters
    pub currency: String,
    /// The amount to charge as a positive integer in the smallest currency unit. (That is, 100 cents to charge $1.00, or 100 to charge JPY 100, a 0-decimal currency).
    /// Note: For details about the smallest currency unit, see Smallest unit of the currency.
    /// More information about this field:
    /// Value range: 1 - unlimited
    pub value: i32,
}

impl From<&PaymentCashier> for OrderAmount {
    fn from(value: &PaymentCashier) -> Self {
        let PaymentCashier {
            currency, amount, ..
        } = value;
        Self {
            value: amount.clone(),
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
    pub order_amount: OrderAmount,
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
        let roi = reference_order_id.unwrap_or(String::from(""));
        let od = order_description.unwrap_or(String::from(""));
        let order_amount = OrderAmount::from(value);
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
