//! Model struct for Invoice type

use serde::{Deserialize, Serialize};

use super::{
    enums::{InvoiceDeliveryMethod, InvoiceStatus, Timezone},
    DateTime, InvoiceAcceptedPaymentMethods, InvoiceCustomField, InvoicePaymentRequest,
    InvoiceRecipient, Money,
};

/// Stores information about an invoice.
///
/// You use the Invoices API to create and manage invoices. For more information, see [Invoices API
/// Overview](https://developer.squareup.com/docs/invoices-api/overview).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Invoice {
    /// **Read only** The Square-assigned ID of the invoice.
    pub id: Option<String>,
    /// The payment methods that customers can use to pay the invoice on the Square-hosted invoice
    /// page. This setting is independent of any automatic payment requests for the invoice.
    ///
    /// This field is required when creating an invoice and must set at least one payment method to
    /// `true`.
    pub accepted_payment_methods: Option<InvoiceAcceptedPaymentMethods>,
    /// **Read only** The timestamp when the invoice was created, in RFC 3339 format.
    pub created_at: Option<DateTime>,
    /// Additional seller-defined fields that are displayed on the invoice. For more information,
    /// see [Custom
    /// fields](https://developer.squareup.com/docs/invoices-api/overview#custom-fields).
    ///
    /// Adding custom fields to an invoice requires an [Invoices Plus
    /// subscription](https://developer.squareup.com/docs/invoices-api/overview#invoices-plus-subscription).
    ///
    /// Max: 2 custom fields
    ///
    /// Max Length: 2
    pub custom_fields: Option<Vec<InvoiceCustomField>>,
    /// The delivery method that Square uses to send the invoice, reminders, and receipts to the
    /// customer. After the invoice is published, Square processes the invoice based on the delivery
    /// method and payment request settings, either immediately or on the `scheduled_at` date, if
    /// specified. For example, Square might send the invoice or receipt for an automatic payment.
    /// For invoices with automatic payments, this field must be set to `EMAIL`.
    ///
    /// One of the following is required when creating an invoice:
    ///
    /// - (Recommended) This `delivery_method` field. To configure an automatic payment, the
    /// `automatic_payment_source` field of the payment request is also required.
    /// - The deprecated `request_method` field of the payment request. Note that `invoice` objects
    /// returned in responses do not include `request_method`.
    pub delivery_method: Option<InvoiceDeliveryMethod>,
    /// The description of the invoice, which is displayed on the invoice.
    ///
    /// Min Length: 1, Max Length: 65536
    pub description: Option<String>,
    /// A user-friendly invoice number that is displayed on the invoice. The value is unique within
    /// a location. If not provided when creating an invoice, Square assigns a value. It increments
    /// from 1 and is padded with zeros making it 7 characters long (for example, 0000001 and
    /// 0000002).
    ///
    /// Min Length: 1, Max Length: 191
    pub invoice_number: Option<String>,
    /// The ID of the location that this invoice is associated with.
    ///
    /// If specified in a `CreateInvoice` request, the value must match the `location_id` of the
    /// associated order.
    ///
    /// Min Length: 1, Max Length: 255
    pub location_id: Option<String>,
    /// Read only The current amount due for the invoice. In addition to the amount due on the next
    /// payment request, this includes any overdue payment amounts.
    pub next_payment_amount_money: Option<Money>,
    /// The ID of the [Order] for which the invoice is created. This field is required when creating
    /// an invoice, and the order must be in the `OPEN` state.
    ///
    /// To view the line items and other information for the associated order, call the
    /// [RetrieveOrder](https://developer.squareup.com/reference/square/orders-api/retrieve-order)
    /// endpoint using the order ID.
    ///
    /// Min Length: 1, Max Length: 255
    pub order_id: Option<String>,
    /// **France only**. The payment terms and conditions that are displayed on the invoice. For
    /// more information, see [Payment
    /// conditions](https://developer.squareup.com/docs/invoices-api/overview#payment-conditions).
    ///
    /// For countries other than France, Square returns an `INVALID_REQUEST_ERROR` with a
    /// `BAD_REQUEST` code and "Payment conditions are not supported for this location's country"
    /// detail if this field is included in `CreateInvoice` or `UpdateInvoice` requests.
    ///
    /// Min Length: 1, Max Length: 2000
    pub payment_conditions: Option<String>,
    /// The payment schedule for the invoice, represented by one or more payment requests that
    /// define payment settings, such as amount due and due date. An invoice supports the following
    /// payment request combinations:
    ///
    /// - One balance
    /// - One deposit with one balance
    /// - 2–12 installments
    /// - One deposit with 2–12 installments
    ///
    /// This field is required when creating an invoice. It must contain at least one payment
    /// request. All payment requests for the invoice must equal the total order amount. For more
    /// information, see [Configuring payment
    /// requests](https://developer.squareup.com/docs/invoices-api/create-publish-invoices#payment-requests).
    ///
    /// Adding `INSTALLMENT` payment requests to an invoice requires an [Invoices Plus
    /// subscription](https://developer.squareup.com/docs/invoices-api/overview#invoices-plus-subscription).
    ///
    /// Min Length: 1, Max Length: 13
    pub payment_requests: Option<Vec<InvoicePaymentRequest>>,
    /// The customer who receives the invoice. This customer data is displayed on the invoice and
    /// used by Square to deliver the invoice.
    ///
    /// This field is required to publish an invoice, and it must specify the `customer_id`.
    pub primary_recipient: Option<InvoiceRecipient>,
    /// **Read only** The URL of the Square-hosted invoice page. After you publish the invoice using
    /// the `PublishInvoice` endpoint, Square hosts the invoice page and returns the page URL in the
    /// response.
    pub public_url: Option<String>,
    /// The date of the sale or the date that the service is rendered, in `YYYY-MM-DD` format. This
    /// field can be used to specify a past or future date which is displayed on the invoice.
    pub sale_or_service_date: Option<String>,
    /// The timestamp when the invoice is scheduled for processing, in RFC 3339 format. After the
    /// invoice is published, Square processes the invoice on the specified date, according to the
    /// delivery method and payment request settings.
    ///
    /// If the field is not set, Square processes the invoice immediately after it is published.
    pub scheduled_at: Option<DateTime>,
    /// **Read only** The status of the invoice.
    pub status: Option<InvoiceStatus>,
    /// Indicates whether to allow a customer to save a credit or debit card as a card on file or a
    /// bank transfer as a bank account on file. If `true`, Square displays a **Save my card on
    /// file** or **Save my bank on file** checkbox on the invoice payment page. Stored payment
    /// information can be used for future automatic payments. The default value is `false`.
    pub store_payment_method_enabled: Option<bool>,
    /// **Read only** The ID of the
    /// [subscription](https://developer.squareup.com/reference/square/objects/Subscription)
    /// associated with the invoice. This field is present only on subscription billing invoices.
    pub subscription_id: Option<String>,
    /// Read only The time zone used to interpret calendar dates on the invoice, such as `due_date`.
    /// When an invoice is created, this field is set to the `timezone` specified for the seller
    /// location. The value cannot be changed.
    pub timezone: Option<Timezone>,
    /// The title of the invoice, which is displayed on the invoice.
    ///
    /// Min Length: 1, Max Length: 255
    pub title: Option<String>,
    /// **Read only** The timestamp when the invoice was last updated, in RFC 3339 format.
    pub updated_at: Option<DateTime>,
    /// The Square-assigned version number, which is incremented each time an update is committed to
    /// the invoice.
    pub version: Option<i32>,
}
