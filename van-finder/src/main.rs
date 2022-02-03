use std::{collections::BTreeMap, path::Path, time::Duration};

use tokio::time::sleep;
use tracing::{debug, info};
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};
use van_finder::{van_summary_html, Error, Site, StoredHighwaterData};

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
        let _ = van_finder(&client).await?;
        sleep(Duration::from_secs(REPEAT_INTERVAL_SECONDS)).await;
    }
}

async fn van_finder(client: &reqwest::Client) -> Result<(), Error> {
    let path = Path::new(FAKE_DB);
    //spawn task for each site that parses/and returns any interesting data
    let mut previous_hw = StoredHighwaterData::read_data(path)
        .await?
        .unwrap_or_else(|| StoredHighwaterData {
            data: BTreeMap::new(),
        });

    let vc_data = previous_hw.data.get(&Site::TheVanCamper).map(|v| v.clone());
    debug!("van camper data: {:?}", vc_data);
    let van_camper_data = van_finder::van_camper(&client, vc_data).await?;
    // repeat with other sites
    //append extra hw data to vec
    previous_hw
        .write_data(path, vec![van_camper_data.highwater.clone()])
        .await?;
    //combine all data into Vec<VanSummary> and email
    if van_camper_data.data.len() > 0 {
        van_finder::send_email(van_summary_html(&van_camper_data.data)).await?;
    } else {
        info!("no new data.. not emailing");
    }

    Ok(())
}

/*
curl 'https://www.vanviewer.com/wp-admin/admin-ajax.php' \
  --data-raw $'action=jet_smart_filters&provider=jet-engine%2Fdefault&query%5B_meta_query_price%7Crange%5D=1000-90000&query%5B_sort_standard%5D=%7B%22orderby%22%3A%22meta_value_num%22%2C%22order%22%3A%22DESC%22%2C%22meta_key%22%3A%22year%22%7D&defaults%5Bpost_status%5D%5B%5D=publish&defaults%5Bpost_type%5D=listings&defaults%5Bposts_per_page%5D=20&defaults%5Bpaged%5D=1&defaults%5Bignore_sticky_posts%5D=1&defaults%5Border%5D=DESC&defaults%5Borderby%5D=date&defaults%5Bmeta_query%5D%5B0%5D%5Bkey%5D=listing-status&defaults%5Bmeta_query%5D%5B0%5D%5Bvalue%5D=Sold&defaults%5Bmeta_query%5D%5B0%5D%5Bcompare%5D=\u0021%3D&defaults%5Bmeta_query%5D%5B0%5D%5Btype%5D=CHAR&settings%5Blisitng_id%5D=38124&settings%5Bcolumns%5D=4&settings%5Bcolumns_tablet%5D=2&settings%5Bcolumns_mobile%5D=1&settings%5Bpost_status%5D%5B%5D=publish&settings%5Buse_random_posts_num%5D=&settings%5Bposts_num%5D=20&settings%5Bmax_posts_num%5D=9&settings%5Bnot_found_message%5D=No+Vans+Available.+Try+adjusting+your+filters+to+show+more+results.&settings%5Bis_masonry%5D=&settings%5Bequal_columns_height%5D=&settings%5Buse_load_more%5D=&settings%5Bload_more_id%5D=&settings%5Bload_more_type%5D=click&settings%5Buse_custom_post_types%5D=&settings%5Bhide_widget_if%5D=&settings%5Bcarousel_enabled%5D=&settings%5Bslides_to_scroll%5D=1&settings%5Barrows%5D=true&settings%5Barrow_icon%5D=fa+fa-angle-left&settings%5Bdots%5D=&settings%5Bautoplay%5D=true&settings%5Bautoplay_speed%5D=5000&settings%5Binfinite%5D=true&settings%5Bcenter_mode%5D=&settings%5Beffect%5D=slide&settings%5Bspeed%5D=500&settings%5Binject_alternative_items%5D=&settings%5Bscroll_slider_enabled%5D=&settings%5Bscroll_slider_on%5D%5B%5D=desktop&settings%5Bscroll_slider_on%5D%5B%5D=tablet&settings%5Bscroll_slider_on%5D%5B%5D=mobile&settings%5Bcustom_query%5D=&props%5Bmax_num_pages%5D=10&props%5Bpage%5D=1' \
  --compressed

$$('div[class="elementor-widget-container"]') -> send this


curl 'https://api.thevancamper.com/posts?%24limit=12&%24skip=0&%24select%5B0%5D=id&%24select%5B1%5D=title&%24select%5B2%5D=price&%24select%5B3%5D=odometer&%24select%5B4%5D=odometerUnit&%24select%5B5%5D=seats&%24select%5B6%5D=sleeps&%24select%5B7%5D=year&%24select%5B8%5D=fuel&%24select%5B9%5D=currency&%24select%5B10%5D=isSold&%24select%5B11%5D=isPending&%24select%5B12%5D=createdAt&%24select%5B13%5D=videoUrl&%24select%5B14%5D=messagingMode&%24eager=%5Bimages%2Cplace%28defaultSelects%29%5D&%24sort%5BisSold%5D=1&%24sort%5BcreatedAt%5D=-1&sleeps%5B%24gte%5D=2&seats%5B%24gte%5D=3&price%5B%24gte%5D=0&price%5B%24lte%5D=9000000&odometer%5B%24gte%5D=0&year%5B%24gte%5D=1940&year%5B%24lte%5D=2022' \
  --compressed

*/
