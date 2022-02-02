mod errors;
mod sites;

use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
};

use chrono::DateTime;
pub use errors::Error;
use serde::{Deserialize, Serialize};
pub use sites::*;
use tracing::warn;

pub type VanTime = DateTime<chrono::Utc>;

pub trait VanDataProvider {
    fn get_data(
        client: &reqwest::Client,
        previous_hw: Option<HighwaterMark>,
    ) -> Result<VanData, Error>;
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredHighwaterData {
    pub data: BTreeMap<Site, HighwaterMark>,
}

impl StoredHighwaterData {
    pub async fn write_data(
        &mut self,
        path: impl AsRef<Path>,
        data: Vec<HighwaterMark>,
    ) -> Result<(), Error> {
        for hw in data {
            if let Some(val) = self.data.get_mut(&hw.site) {
                val.created_at = hw.created_at;
                val.id = hw.id;
            } else {
                self.data.insert(hw.site.clone(), hw);
            }
        }
        let marshall = serde_json::to_string(self)?;
        tokio::fs::write(path, marshall.as_bytes()).await?;
        Ok(())
    }

    pub async fn read_data(path: impl AsRef<Path>) -> Result<Option<StoredHighwaterData>, Error> {
        let hw = tokio::fs::read(path).await?;
        let val = serde_json::from_slice(&hw).unwrap_or_else(|err| {
            warn!("failed to parse data: {:?}", err);
            None
        });
        Ok(val)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct HighwaterMark {
    pub site: Site,
    pub created_at: VanTime,
    pub id: String,
}

impl HighwaterMark {
    pub fn default_datetime() -> VanTime {
        chrono::Utc::now() - chrono::Duration::days(365)
    }
}
#[derive(Clone, Debug)]
pub struct VanSummary {
    pub url: String,
    pub name: String,
    pub price: String,
    pub miles: String,
}
