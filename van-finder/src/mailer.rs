use crate::Error;

use chrono::Local;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use once_cell::sync::Lazy;
use tracing::info;

static GMAIL_USER: Lazy<String> =
    Lazy::new(|| std::env::var("GMAIL_USER").expect("GMAIL_USER required"));
// yeah, this should be in secrets manager.. but this is a trivial program
// you need to allow 'less secure apps' so you can avoid 2factor auth
// https://webewizard.com/2019/09/17/Using-Lettre-With-Gmail/
static GMAIL_PASSWORD: Lazy<String> =
    Lazy::new(|| std::env::var("GMAIL_PASSWORD").expect("GMAIL_PASSWORD required"));

static RECIPIENT_EMAIL: Lazy<String> =
    Lazy::new(|| std::env::var("RECIPIENT_EMAIL").expect("RECIPIENT_EMAIL required"));

//TODO: accept html instead of string
pub async fn send_email(msg: String) -> Result<(), Error> {
    let email = Message::builder()
        .from(GMAIL_USER.parse().unwrap())
        .to(RECIPIENT_EMAIL.parse().unwrap())
        .subject(format!("Van Finder: {}", Local::now().date()))
        .singlepart(SinglePart::html(msg))?;

    let creds = Credentials::new(GMAIL_USER.to_string(), GMAIL_PASSWORD.to_string());

    // Open a remote connection to gmail (it uses the submissions port 465)
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    let res = mailer.send(&email)?;
    info!("Sent email! {:?}", res);
    Ok(())
}
