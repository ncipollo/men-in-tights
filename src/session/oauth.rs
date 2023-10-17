use crate::session::challenge::ChallengeType;
use crate::session::challenge::ChallengeType::SMS;
use serde::{Deserialize, Serialize};

const CLIENT_ID: &str = "c82SH0WZOsabOXGP2sxqcj34FxkvfnWRZBKlBjFS";
const EXPIRATION_TIME: i64 = 734000;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OAuthLoginRequest {
    challenge_type: ChallengeType,
    client_id: String,
    device_token: String,
    expires_in: i64,
    grant_type: String,
    password: String,
    scope: String,
    username: String,
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

#[cfg(test)]
mod test {
    use indoc::indoc;
    use crate::session::challenge::ChallengeType::SMS;
    use crate::session::oauth::login_request;

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
}

// "password": self.password,
// "username": self.username,
// "grant_type": "password",
// "client_id": CLIENT_ID,
// "expires_in": EXPIRATION_TIME,
// "scope": "internal",
// "device_token": self.device_token,
// "challenge_type": self.challenge_type,
