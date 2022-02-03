use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{Error, HighwaterMark, SaleStatus, Site, VanData, VanSummary};

pub async fn van_camper(
    client: &reqwest::Client,
    previous: Option<HighwaterMark>,
) -> Result<VanData, Error> {
    let mut skip = 0;
    let limit = 12;
    let last_id = previous.map(|i| i.id).unwrap_or("".into());
    let mut new_hw: Option<HighwaterMark> = None;
    let mut new_van_info: Vec<VanSummary> = Vec::new();
    loop {
        let query = format!("%24limit={}&%24skip={}&%24select%5B0%5D=id&%24select%5B1%5D=title&%24select%5B2%5D=price&%24select%5B3%5D=odometer&%24select%5B4%5D=odometerUnit&%24select%5B5%5D=seats&%24select%5B6%5D=sleeps&%24select%5B7%5D=year&%24select%5B8%5D=fuel&%24select%5B9%5D=currency&%24select%5B10%5D=isSold&%24select%5B11%5D=isPending&%24select%5B12%5D=createdAt&%24select%5B13%5D=videoUrl&%24select%5B14%5D=messagingMode&%24eager=%5Bimages%2Cplace%28defaultSelects%29%5D&%24sort%5BisSold%5D=1&%24sort%5BcreatedAt%5D=-1&sleeps%5B%24gte%5D=2&seats%5B%24gte%5D=3&price%5B%24gte%5D=0&price%5B%24lte%5D=9000000", limit, skip);
        let url = format!("https://api.thevancamper.com/posts?{}", query);
        let resp = client.get(url).send().await?;
        let json: serde_json::Value = resp.json().await?;
        let van_data: crate::VanCamperResp = serde_json::from_value(json)?;

        let mut last_loop = van_data.data.len() < 1 || van_data.total < skip;
        // check if we are going past our previous HW mark and flag as last loop
        if new_hw.is_none() {
            let newest = van_data.data.first();
            new_hw = newest.map_or_else(
                || {
                    Some(HighwaterMark {
                        site: Site::TheVanCamper,
                        created_at: HighwaterMark::default_datetime(),
                        id: "".into(),
                    })
                },
                |d| {
                    Some(HighwaterMark {
                        site: Site::TheVanCamper,
                        created_at: d.created_at.parse::<_>().unwrap_or_else(|err| {
                            warn!("Failed to parse highwater timestamp: {:?}", err);
                            HighwaterMark::default_datetime()
                        }),
                        id: d.id.to_string(),
                    })
                },
            );
        }

        for van in van_data.data {
            if van.id.to_string() == last_id {
                last_loop = true;
                break;
            } else {
                let status = if van.is_sold {
                    SaleStatus::IsSold
                } else if van.is_pending {
                    SaleStatus::IsPending
                } else {
                    SaleStatus::ForSale
                };

                new_van_info.push(VanSummary {
                    url: format!("https://thevancamper.com/post/{}", van.id),
                    name: van.title,
                    price: van.display_price,
                    miles: format!("{} {:?}", van.odometer, van.odometer_unit),
                    status,
                });
            }
        }

        if last_loop {
            break;
        }
        skip += limit;
    }
    Ok(VanData {
        site: Site::TheVanCamper,
        highwater: new_hw.unwrap(),
        data: new_van_info,
    })
}

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
    pub place: Place,
    #[serde(rename = "displayPrice")]
    pub display_price: String,
    #[serde(rename = "pricingInfo")]
    pub pricing_info: PricingInfo,
    pub slug: String,
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
pub enum OdometerUnit {
    #[serde(rename = "miles")]
    Miles,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CountryCode {
    #[serde(rename = "US")]
    Us,
}
