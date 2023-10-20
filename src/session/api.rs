use crate::error::RobinhoodError;
use crate::session::oauth::{OAuthLoginRequest, OAuthResponse};
use crate::{headers, urls};
use reqwest::Client;

pub struct SessionApi {
    client: Client,
}

impl SessionApi {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn login(&self, request: OAuthLoginRequest) -> Result<String, RobinhoodError> {
        let oauth = self
            .client
            .post(urls::api("oauth2/token/"))
            .json(&request)
            .headers(headers::standard())
            .send()
            .await?
            .text()
            // .json::<OAuthResponse>()
            .await?;
        Ok(oauth)
    }
}
