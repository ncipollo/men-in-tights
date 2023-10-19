use reqwest::Client;

pub struct SessionApi {
    client: Client
}

impl SessionApi {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}