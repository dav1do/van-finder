use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};
use tracing::{info, warn};

use crate::{Error, Site, VanTime};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredHighwaterData {
    pub data: BTreeMap<Site, HighwaterMark>,
}

impl StoredHighwaterData {
    pub async fn write_data(&mut self, path: &Path, data: Vec<HighwaterMark>) -> Result<(), Error> {
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

    pub async fn read_data(path: &Path) -> Result<Option<StoredHighwaterData>, Error> {
        let hw = match tokio::fs::read(path.clone()).await {
            Ok(data) => data,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    info!("File not found... creating {:?}", path);
                    tokio::fs::File::create(path).await?;
                    vec![]
                }
                e => {
                    return Err(Error::Custom(format!(
                        "Error reading {:?}: {:?}",
                        path.to_str(),
                        e
                    )))
                }
            },
        };

        let val = serde_json::from_slice(&hw).unwrap_or_else(|err| {
            warn!("failed to parse data (okay if new file): {:?}", err);
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
