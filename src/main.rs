use men_in_tights::error::RobinhoodError;
use men_in_tights::headers;
use std::io;
use std::io::Read;
use reqwest::Client;
use men_in_tights::session::api::SessionApi;
use men_in_tights::session::challenge::ChallengeType::SMS;
use men_in_tights::session::oauth;
use men_in_tights::session::oauth::OAuthLoginRequest;

#[tokio::main]
async fn main() -> Result<(), RobinhoodError> {
    println!("Enter username:");
    let mut username_input = String::new();
    io::stdin()
        .read_line(&mut username_input)
        .expect("failed to read username");
    let username = username_input.trim();


    println!("Enter password:");
    let mut password_input = String::new();
    io::stdin()
        .read_line(&mut password_input)
        .expect("failed to read password");
    let password = password_input.trim();

    let request = oauth::login_request(username, &password, SMS, "njc_test");
    let session = SessionApi::new(Client::new());
    let response = session.login(request).await?;

    println!("response: {:?}", response);
    Ok(())
}
