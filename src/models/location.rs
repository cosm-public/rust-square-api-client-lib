//! Model struct for Location type

use serde::{Deserialize, Serialize};

use super::{
    enums::{
        Country, Currency, Language, LocationCapability, LocationStatus, LocationType, Timezone,
    },
    Address, BusinessHours, Coordinates, DateTime, TaxIds,
};

/// Represents one of a business' [locations](https://developer.squareup.com/docs/locations-api).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Location {
    /// **Read only** A short generated string of letters and numbers that uniquely identifies this
    /// location instance.
    ///
    /// Max Length 32
    pub id: Option<String>,
    /// The name of the location. This information appears in the Seller Dashboard as the nickname.
    /// A location name must be unique within a seller account.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// The physical address of the location.
    pub address: Option<Address>,
    /// The [IANA time zone](https://www.iana.org/time-zones) identifier for the time zone of the
    /// location. For example, `America/Los_Angeles`.
    pub timezone: Option<Timezone>,
    /// **Read only** The Square features that are enabled for the location.
    pub capabilities: Option<Vec<LocationCapability>>,
    /// The status of the location.
    pub status: Option<LocationStatus>,
    /// **Read only** The time when the location was created, in RFC 3339 format. For more
    /// information, see
    /// [Working with Dates](https://developer.squareup.com/docs/build-basics/working-with-dates).
    pub created_at: Option<DateTime>,
    /// **Read only** The ID of the merchant that owns the location.
    ///
    /// Max Length 32
    pub merchant_id: Option<String>,
    /// **Read only** The country of the location, in the two-letter format of ISO 3166. For
    /// example, `US` or `JP`.
    pub country: Option<Country>,
    /// The language associated with the location, in [BCP 47
    /// format](https://tools.ietf.org/html/bcp47#appendix-A).
    /// For more information, see [Location language
    /// code](https://developer.squareup.com/docs/build-basics/general-considerations/language-preferences).
    pub language_code: Option<Language>,
    /// **Read only** The currency used for all transactions at this location, in ISO 4217 format.
    /// For example, the currency code for US dollars is `USD`.
    pub currency: Option<Currency>,
    /// The phone number of the location. For example, `+1 855-700-6000`.
    ///
    /// Max Length 17
    pub phone_number: Option<String>,
    /// The name of the location's overall business. This name is present on receipts and other
    /// customer-facing branding.
    ///
    /// Max Length 255
    pub business_name: Option<String>,
    /// The type of the location.
    pub r#type: Option<LocationType>,
    /// The website URL of the location. For example, `https://squareup.com`.
    ///
    /// Max Length 255
    pub website_url: Option<String>,
    /// The hours of operation for the location.
    pub business_hours: Option<BusinessHours>,
    /// The email address of the location. This can be unique to the location and is not always the
    /// email address for the business owner or administrator.
    ///
    /// Max Length 255
    pub business_email: Option<String>,
    /// The description of the location. For example, `Main Street location`.
    ///
    /// Max Length 1024
    pub description: Option<String>,
    /// The Twitter username of the location without the '@' symbol. For example, `Square`.
    ///
    /// Min Length 1 Max Length 15
    pub twitter_username: Option<String>,
    /// The Instagram username of the location without the '@' symbol. For example, `square`.
    ///
    /// Min Length 1 Max Length 30
    pub instagram_username: Option<String>,
    /// The Facebook profile URL of the location. The URL should begin with 'facebook.com/'. For
    /// example, `https://www.facebook.com/square`.
    ///
    /// Max Length 255
    pub facebook_url: Option<String>,
    /// The physical coordinates (latitude and longitude) of the location.
    pub coordinates: Option<Coordinates>,
    /// **Read only** The URL of the logo image for the location. When configured in the Seller
    /// Dashboard (Receipts section), the logo appears on transactions (such as receipts and
    /// invoices) that Square generates on behalf of the seller. This image should have a roughly
    /// square (1:1) aspect ratio and should be at least 200x200 pixels.
    ///
    /// Max Length 255
    pub logo_url: Option<String>,
    /// **Read only** The URL of the Point of Sale background image for the location.
    ///
    /// Max Length 255
    pub pos_background_url: Option<String>,
    /// A four-digit number that describes the kind of goods or services sold at the location. The
    /// [merchant category code
    /// (MCC)](https://developer.squareup.com/docs/locations-api#initialize-a-merchant-category-code)
    /// of the location as standardized by ISO 18245. For example, `5045`, for a location that sells
    /// computer goods and software.
    ///
    /// Min Length 4 Max Length 4
    pub mcc: Option<String>,
    /// **Read only** The URL of a full-format logo image for the location. When configured in the
    /// Seller Dashboard (Receipts section), the logo appears on transactions (such as receipts and
    /// invoices) that Square generates on behalf of the seller. This image can be wider than it is
    /// tall and should be at least 1280x648 pixels.
    pub full_format_logo_url: Option<String>,
    /// The tax IDs for this location.
    pub tax_ids: Option<TaxIds>,
}
