use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VanCamperResp {
    pub total: i64,
    pub limit: i64,
    pub skip: i64,
    pub data: Vec<Datum>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Datum {
    pub id: i64,
    pub title: String,
    pub price: i64,
    pub odometer: i64,
    #[serde(rename = "odometerUnit")]
    pub odometer_unit: OdometerUnit,
    pub seats: i64,
    pub sleeps: i64,
    pub year: i64,
    pub fuel: Fuel,
    pub currency: Currency,
    #[serde(rename = "isSold")]
    pub is_sold: bool,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "videoUrl")]
    pub video_url: Option<String>,
    #[serde(rename = "messagingMode")]
    pub messaging_mode: MessagingMode,
    pub place: Place,
    pub images: Vec<Image>,
    #[serde(rename = "displayPrice")]
    pub display_price: String,
    #[serde(rename = "pricingInfo")]
    pub pricing_info: PricingInfo,
    pub slug: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub bucket: Bucket,
    pub id: i64,
    pub path: String,
    #[serde(rename = "postSort")]
    pub post_sort: i64,
    pub url: String,
    pub alts: Alts,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alts {
    #[serde(rename = "100")]
    pub the_100: String,
    #[serde(rename = "400")]
    pub the_400: String,
    #[serde(rename = "600")]
    pub the_600: String,
    #[serde(rename = "800")]
    pub the_800: String,
    #[serde(rename = "1400")]
    pub the_1400: String,
    pub twitter: String,
    pub og: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Place {
    pub id: i64,
    #[serde(rename = "countryCode")]
    pub country_code: CountryCode,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[serde(rename = "placeName")]
    pub place_name: String,
    #[serde(rename = "adminName1")]
    pub admin_name1: String,
    #[serde(rename = "adminCode1")]
    pub admin_code1: String,
    pub long: f64,
    pub lat: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PricingInfo {
    pub amount: i64,
    pub currency: Currency,
    pub precision: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    Usd,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Fuel {
    #[serde(rename = "diesel")]
    Diesel,
    #[serde(rename = "gasoline")]
    Gasoline,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Bucket {
    #[serde(rename = "vancamp")]
    Vancamp,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessagingMode {
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "on")]
    On,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OdometerUnit {
    #[serde(rename = "miles")]
    Miles,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CountryCode {
    #[serde(rename = "US")]
    Us,
}
