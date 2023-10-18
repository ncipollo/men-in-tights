use crate::session::challenge::ChallengeType;
use serde::{Deserialize, Serialize};

const CLIENT_ID: &str = "c82SH0WZOsabOXGP2sxqcj34FxkvfnWRZBKlBjFS";
const EXPIRATION_TIME: i64 = 734000;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OAuthLoginRequest {
    pub challenge_type: ChallengeType,
    pub client_id: String,
    pub device_token: String,
    pub expires_in: i64,
    pub grant_type: String,
    pub password: String,
    pub scope: String,
    pub username: String,
}

pub fn login_request(
    username: &str,
    password: &str,
    challenge_type: ChallengeType,
    device_token: &str,
) -> OAuthLoginRequest {
    OAuthLoginRequest {
        challenge_type,
        client_id: CLIENT_ID.to_string(),
        device_token: device_token.to_string(),
        expires_in: EXPIRATION_TIME,
        grant_type: "password".to_string(),
        password: password.to_string(),
        scope: "internal".to_string(),
        username: username.to_string(),
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OAuthRefreshRequest {
    pub client_id: String,
    pub expires_in: i64,
    pub grant_type: String,
    pub refresh_token: String,
    pub scope: String,
}

pub fn refresh_request(refresh_token: &str) -> OAuthRefreshRequest {
    OAuthRefreshRequest {
        client_id: CLIENT_ID.to_string(),
        expires_in: EXPIRATION_TIME,
        grant_type: "refresh_token".to_string(),
        refresh_token: refresh_token.to_string(),
        scope: "internal".to_string(),
    }
}

pub struct OAuthResponse {

}

#[cfg(test)]
mod test {
    use crate::session::challenge::ChallengeType::SMS;
    use crate::session::oauth::{login_request, refresh_request};
    use indoc::indoc;

    #[test]
    fn login_request_serialize() {
        let request = login_request("username", "password", SMS, "device");
        let json = serde_json::to_string_pretty(&request).expect("json serialize failed");
        let expected = indoc! {r#"
        {
          "challenge_type": "SMS",
          "client_id": "c82SH0WZOsabOXGP2sxqcj34FxkvfnWRZBKlBjFS",
          "device_token": "device",
          "expires_in": 734000,
          "grant_type": "password",
          "password": "password",
          "scope": "internal",
          "username": "username"
        }"#};
        assert_eq!(json, expected)
    }

    #[test]
    fn refresh_request_serialize() {
        let request = refresh_request("refresh");
        let json = serde_json::to_string_pretty(&request).expect("json serialize failed");
        let expected = indoc! {r#"
        {
          "client_id": "c82SH0WZOsabOXGP2sxqcj34FxkvfnWRZBKlBjFS",
          "expires_in": 734000,
          "grant_type": "refresh_token",
          "refresh_token": "refresh",
          "scope": "internal"
        }"#};
        assert_eq!(json, expected)
    }
}
