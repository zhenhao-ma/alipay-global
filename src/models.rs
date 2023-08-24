use serde::Serialize;

/// Minimum Information to initialize a payment cashier
#[derive(Serialize)]
pub struct PaymentCashier {
    pub payment_request_id: String,
    pub currency: String,
    pub amount: i32,
    pub reference_order_id: Option<String>,
    pub order_description: Option<String>,
    pub terminal_type: Option<String>,
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
    pub productCode: String,
    /// The unique ID assigned by a merchant to identify a payment request. Alipay uses this field for idempotence control.
    /// More information about this field:
    /// This field is an API idempotency field. For payment requests that are initiated with the same value of paymentRequestId and reach a final status of S or F, the same result is to be returned for the request.
    /// Maximum length: 64 characters
    pub paymentRequestId: String,
    pub order: Order,
    pub paymentAmount: OrderAmount,
    pub paymentMethod: PaymentMethod,
    pub paymentRedirectUrl: String,
    pub paymentNotifyUrl: String,
    pub settlementStrategy: SettlementStrategy,
}

impl From<PaymentCashier> for PaymentCashierRequest {
    fn from(value: PaymentCashier) -> Self {

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
pub struct Env {
    /// Terminal type of which the merchant service applies to. Valid values are:
    /// WEB: The client-side terminal type is a website, which is opened via a PC browser.
    /// WAP: The client-side terminal type is an H5 page, which is opened via a mobile browser.
    /// APP: The client-side terminal type is a mobile application.
    /// MINI_APP: The terminal type of the merchant side is a mini program on the mobile phone.  
    terminalType: TerminalType,
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
/// The payment method that is used to collect the payment by the merchant or acquirer.
/// skip attributes
/// - paymentMethodId
/// - paymentMethodMetaData
/// - customerId
/// - extendInfo
#[derive(Serialize)]
pub struct PaymentMethod {
    /// The payment method type that is included in payment method options. By specifying the value of this parameter, you can receive the cashier URL of the specified payment method returned by Alipay. See Payment methods to check the valid values.
    /// More information about this field:
    /// Maximum length: 64 characters
    pub paymentMethodType: String,
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
pub struct Order {
    /// The order amount of the merchant that directly provides services or goods to the customer. This field is used for user consumption records display or payment results page.
    pub orderAmount: OrderAmount,
    pub orderDescription: String,
    pub referenceOrderId: String,
}
/// The settlement strategy for the payment request.
#[derive(Serialize)]
pub struct SettlementStrategy {
    /// The ISO currency code of the currency that the merchant wants to be settled against. The field is required if the merchant signed up for multiple currencies to settle.
    settlementCurrency: String,
}
