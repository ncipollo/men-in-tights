use reqwest::{Client, Error};
use men_in_tights::headers;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let resp = client.get("https://api.robinhood.com/quotes/vti")
        .headers(headers::standard())
        .send()
        .await?
        .text()
        .await?;

    println!("response: {resp}");

    // client.post("https://api.robinhood.com/api-token-auth/")

    println!("we're men, men in tights!");
    Ok(())
}