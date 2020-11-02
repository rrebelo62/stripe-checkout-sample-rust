use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CreateCheckoutSessionResponse{
	pub session_id:String,
}

#[derive(Deserialize)]
pub struct EventRequest{
	pub id: String,
	pub idempotency_key:String
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct StripeEvent {
	pub id : String,
	pub object : String,
	pub account : String,
	pub api_version : String,
	pub date_time : String,
	pub event_data : Vec<u8>,
	pub livemode: bool,
	pub pending_webhooks: i64,
	pub request:EventRequest,
	pub Type : String,
}
#[derive(Deserialize)]
pub struct CustomerPortalSession{
	pub id: String,
	pub object: String,
	pub created: u64,
	pub customer: String,
	pub livemode: bool,
	pub return_url: String,
	pub url: String
}

// The structure below was extracted and adapted from the crate stripe-rust.
// Some of it's fields are changed to just Strings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSession {
    /// Unique identifier for the object.
    ///
    /// Used to pass to `redirectToCheckout` in Stripe.js.
    pub id: String,

    /// The value (`auto` or `required`) for whether Checkout collected the
    /// customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<String>,

    /// The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: String,

    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the session with your internal systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,

    /// The ID of the customer for this session.
    ///
    /// A new customer will be created unless an existing customer was provided in when the session was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file.
    ///
    /// To access information about the customer once a session is complete, use the `customer` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,

    /// The line items, plans, or SKUs purchased by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_items: Option<Vec<String>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// The mode of the Checkout Session, one of `payment`, `setup`, or `subscription`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// The ID of the PaymentIntent for `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,

    /// A list of the types of payment methods (e.g.
    ///
    /// card) this Checkout Session is allowed to accept.
    pub payment_method_types: Vec<String>,

    /// The ID of the SetupIntent if mode was set to `setup`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<String>,

    /// Describes the type of transaction being performed by Checkout in order
    /// to customize relevant text on the page, such as the submit button.
    /// `submit_type` can only be specified on Checkout Sessions using line
    /// items or a SKU, but not Checkout Sessions for subscriptions.
    ///
    /// Supported values are `auto`, `book`, `donate`, or `pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<String>,

    /// The ID of the subscription created if one or more plans were provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    /// The URL the customer will be directed to after the payment or
    /// subscription creation is successful.
    pub success_url: String,
}
