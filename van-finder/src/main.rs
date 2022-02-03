use std::{collections::BTreeMap, time::Duration};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tokio::{fs, signal, time::sleep};
use tracing::{error, info, warn};
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};
use van_finder::{Error, HighwaterMark, Site, StoredHighwaterData, VanData, VanSummary};

static FAKE_DB: &'static str = "./highwater_data.json";
static REPEAT_INTERVAL_SECONDS: u64 = 3600;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting...");
    dotenv::dotenv().unwrap();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .finish()
        .init();

    info!("Initialized... ");

    loop {
        let hw = van_finder(&client).await?;
        sleep(Duration::from_secs(REPEAT_INTERVAL_SECONDS)).await;
    }
}

async fn van_finder(client: &reqwest::Client) -> Result<(), Error> {
    info!("we made it...");
    //spawn task for each site that parses/and returns any interesting data
    let mut previous_hw = StoredHighwaterData::read_data(FAKE_DB)
        .await?
        .unwrap_or_else(|| StoredHighwaterData {
            data: BTreeMap::new(),
        });

    let vc_data = previous_hw.data.get(&Site::TheVanCamper).map(|v| v.clone());
    let res = van_camper(&client, vc_data).await?;
    previous_hw
        .write_data(FAKE_DB, vec![res.highwater.clone()])
        .await?;
    van_finder::send_email(res.to_html()).await?;

    Ok(())
}

async fn van_camper(
    client: &reqwest::Client,
    previous: Option<HighwaterMark>,
) -> Result<VanData, Error> {
    let host = "https://api.thevancamper.com";
    let api = "posts";
    let mut skip = 0;
    let limit = 12;
    let mut new_hw: Option<HighwaterMark> = None;
    let mut new_van_info: Vec<VanSummary> = Vec::new();
    loop {
        let query = format!("%24limit={}&%24skip={}&%24select%5B0%5D=id&%24select%5B1%5D=title&%24select%5B2%5D=price&%24select%5B3%5D=odometer&%24select%5B4%5D=odometerUnit&%24select%5B5%5D=seats&%24select%5B6%5D=sleeps&%24select%5B7%5D=year&%24select%5B8%5D=fuel&%24select%5B9%5D=currency&%24select%5B10%5D=isSold&%24select%5B11%5D=isPending&%24select%5B12%5D=createdAt&%24select%5B13%5D=videoUrl&%24select%5B14%5D=messagingMode&%24eager=%5Bimages%2Cplace%28defaultSelects%29%5D&%24sort%5BisSold%5D=1&%24sort%5BcreatedAt%5D=-1&sleeps%5B%24gte%5D=2&seats%5B%24gte%5D=3&price%5B%24gte%5D=0&price%5B%24lte%5D=9000000", limit, skip);
        let url = format!("{}/{}?{}", host, api, query);
        let resp = client.get(url).send().await?;
        let json: serde_json::Value = resp.json().await?;
        let van_data: van_finder::VanCamperResp = serde_json::from_value(json)?;

        let last_loop = van_data.data.len() < 1 || van_data.total < skip;
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

        let van_summaries = van_data
            .data
            .into_iter()
            .map(|van| VanSummary {
                url: format!("https://thevancamper.com/post/{}", van.id),
                name: van.title,
                price: van.display_price,
                miles: format!("{} {:?}", van.odometer, van.odometer_unit),
            })
            .collect::<Vec<_>>();

        new_van_info.extend(van_summaries);

        if last_loop {
            break;
        }
        skip += limit;
    }
    info!("{:?}", new_van_info);
    info!("{:?}", new_hw);
    Ok(VanData {
        site: Site::TheVanCamper,
        highwater: new_hw.unwrap(),
        data: new_van_info,
    })
}

/*
curl 'https://www.vanviewer.com/wp-admin/admin-ajax.php' \
  --data-raw $'action=jet_smart_filters&provider=jet-engine%2Fdefault&query%5B_meta_query_price%7Crange%5D=1000-90000&query%5B_sort_standard%5D=%7B%22orderby%22%3A%22meta_value_num%22%2C%22order%22%3A%22DESC%22%2C%22meta_key%22%3A%22year%22%7D&defaults%5Bpost_status%5D%5B%5D=publish&defaults%5Bpost_type%5D=listings&defaults%5Bposts_per_page%5D=20&defaults%5Bpaged%5D=1&defaults%5Bignore_sticky_posts%5D=1&defaults%5Border%5D=DESC&defaults%5Borderby%5D=date&defaults%5Bmeta_query%5D%5B0%5D%5Bkey%5D=listing-status&defaults%5Bmeta_query%5D%5B0%5D%5Bvalue%5D=Sold&defaults%5Bmeta_query%5D%5B0%5D%5Bcompare%5D=\u0021%3D&defaults%5Bmeta_query%5D%5B0%5D%5Btype%5D=CHAR&settings%5Blisitng_id%5D=38124&settings%5Bcolumns%5D=4&settings%5Bcolumns_tablet%5D=2&settings%5Bcolumns_mobile%5D=1&settings%5Bpost_status%5D%5B%5D=publish&settings%5Buse_random_posts_num%5D=&settings%5Bposts_num%5D=20&settings%5Bmax_posts_num%5D=9&settings%5Bnot_found_message%5D=No+Vans+Available.+Try+adjusting+your+filters+to+show+more+results.&settings%5Bis_masonry%5D=&settings%5Bequal_columns_height%5D=&settings%5Buse_load_more%5D=&settings%5Bload_more_id%5D=&settings%5Bload_more_type%5D=click&settings%5Buse_custom_post_types%5D=&settings%5Bhide_widget_if%5D=&settings%5Bcarousel_enabled%5D=&settings%5Bslides_to_scroll%5D=1&settings%5Barrows%5D=true&settings%5Barrow_icon%5D=fa+fa-angle-left&settings%5Bdots%5D=&settings%5Bautoplay%5D=true&settings%5Bautoplay_speed%5D=5000&settings%5Binfinite%5D=true&settings%5Bcenter_mode%5D=&settings%5Beffect%5D=slide&settings%5Bspeed%5D=500&settings%5Binject_alternative_items%5D=&settings%5Bscroll_slider_enabled%5D=&settings%5Bscroll_slider_on%5D%5B%5D=desktop&settings%5Bscroll_slider_on%5D%5B%5D=tablet&settings%5Bscroll_slider_on%5D%5B%5D=mobile&settings%5Bcustom_query%5D=&props%5Bmax_num_pages%5D=10&props%5Bpage%5D=1' \
  --compressed

$$('div[class="elementor-widget-container"]') -> send this


curl 'https://api.thevancamper.com/posts?%24limit=12&%24skip=0&%24select%5B0%5D=id&%24select%5B1%5D=title&%24select%5B2%5D=price&%24select%5B3%5D=odometer&%24select%5B4%5D=odometerUnit&%24select%5B5%5D=seats&%24select%5B6%5D=sleeps&%24select%5B7%5D=year&%24select%5B8%5D=fuel&%24select%5B9%5D=currency&%24select%5B10%5D=isSold&%24select%5B11%5D=isPending&%24select%5B12%5D=createdAt&%24select%5B13%5D=videoUrl&%24select%5B14%5D=messagingMode&%24eager=%5Bimages%2Cplace%28defaultSelects%29%5D&%24sort%5BisSold%5D=1&%24sort%5BcreatedAt%5D=-1&sleeps%5B%24gte%5D=2&seats%5B%24gte%5D=3&price%5B%24gte%5D=0&price%5B%24lte%5D=9000000&odometer%5B%24gte%5D=0&year%5B%24gte%5D=1940&year%5B%24lte%5D=2022' \
  --compressed

*/
