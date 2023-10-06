mod errors;
mod highwater;
mod mailer;
mod sites;

use std::fmt::Display;

use chrono::DateTime;
pub use errors::Error;
pub use highwater::*;
pub use mailer::send_email;
use serde::{Deserialize, Serialize};
pub use sites::*;
pub type VanTime = DateTime<chrono::Utc>;

pub trait VanDataProvider {
    fn get_data(
        client: &reqwest::Client,
        previous_hw: Option<HighwaterMark>,
    ) -> Result<VanData, Error>;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VanData {
    pub site: Site,
    pub highwater: HighwaterMark,
    pub data: Vec<VanSummary>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum Site {
    TheVanCamper,
    VanLifeTrader,
    VanViewer,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Ord)]
pub struct VanSummary {
    pub url: String,
    pub name: String,
    pub summary: String,
    pub price: i32,
    pub miles: i32,
    pub status: SaleStatus,
}

impl PartialOrd for VanSummary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.status.partial_cmp(&other.status) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.miles.partial_cmp(&other.miles) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.price.partial_cmp(&other.price)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum SaleStatus {
    ForSale,
    IsPending,
    IsSold,
    Unknown,
}

impl Display for SaleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaleStatus::IsSold => write!(f, "Sold"),
            SaleStatus::IsPending => write!(f, "Sale Pending"),
            SaleStatus::ForSale => write!(f, "For Sale"),
            SaleStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

impl VanSummary {
    pub fn to_html(&self) -> String {
        let val = format!(
            r#"<div><a href="{}">{}</a><p> {} ({})</p></div>"#,
            self.url, self.name, self.summary, self.status
        );
        val
    }
}

pub fn van_summary_html(van_data: &[VanSummary]) -> String {
    let s = van_data
        .iter()
        .map(|v| format!("<li>{}</li>", v.to_html()))
        .collect::<Vec<_>>()
        .join("");
    let val = format!("<html>{}</html>", s);
    val
}
