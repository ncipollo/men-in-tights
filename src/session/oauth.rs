use crate::session::challenge::{Challenge, ChallengeType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Models and IDs sourced from:
// https://github.com/robinhood-unofficial/pyrh/blob/ceb57857f1391dc3d5c13bc2ef87cf6a5b512492/pyrh/models/sessionmanager.py#L1

const CLIENT_ID: &str = "c82SH0WZOsabOXGP2sxqcj34FxkvfnWRZBKlBjFS";
const EXPIRATION_TIME: i64 = 734000;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OAuthLoginRequest {
    pub challenge_type: ChallengeType,
    pub client_id: String,
    pub device_token: String,
    pub expires_in: i64,
    pub grant_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
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
        mfa_code: None,
        password: password.to_string(),
        scope: "internal".to_string(),
        username: username.to_string(),
    }
}

pub fn mfa_request(mfa_code: String, request: OAuthLoginRequest) -> OAuthLoginRequest {
    OAuthLoginRequest {
        mfa_code: Some(mfa_code),
        ..request
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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OAuthResponse {
    pub detail: String,
    pub challenge: Option<Challenge>,

    #[serde(default)]
    pub mfa_required: bool,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: i64,

    #[serde(flatten)]
    pub extra_fields: HashMap<String, Value>,
}

#[cfg(test)]
mod test {
    use crate::session::challenge::{Challenge, ChallengeStatus, ChallengeType};
    use crate::session::oauth::{login_request, refresh_request, OAuthResponse, mfa_request};
    use chrono::DateTime;
    use indoc::indoc;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn login_request_serialize() {
        let request = login_request("username", "password", ChallengeType::SMS, "device");
        let json = serde_json::to_string_pretty(&request).expect("json serialize failed");
        let expected = indoc! {r#"
        {
          "challenge_type": "sms",
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
    fn mfa_request_serialize() {
        let login_request = login_request("username", "password", ChallengeType::SMS, "device");
        let mfa_request = mfa_request("code".to_string(), login_request);
        let json = serde_json::to_string_pretty(&mfa_request).expect("json serialize failed");
        let expected = indoc! {r#"
        {
          "challenge_type": "sms",
          "client_id": "c82SH0WZOsabOXGP2sxqcj34FxkvfnWRZBKlBjFS",
          "device_token": "device",
          "expires_in": 734000,
          "grant_type": "password",
          "mfa_code": "code",
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

    #[test]
    fn oauth_response_deserialize() {
        let json = indoc! {
            r#"
            {
              "detail": "detail",
              "challenge": {
                "id": "challenge_id",
                "user": "user",
                "type": "sms",
                "status": "issued",
                "expires_at": "2023-10-18T10:59:50.159306Z"
              },
              "mfa_required": true,
              "access_token": "access",
              "refresh_token": "refresh",
              "expires_in": 100,
              "some_other_field": "extra"
            }
            "#
        };

        let oauth: OAuthResponse = serde_json::from_str(json).expect("failed to deserialize");

        let expected_challenge = Challenge {
            id: "challenge_id".to_string(),
            user: "user".to_string(),
            challenge_type: ChallengeType::SMS,
            status: ChallengeStatus::ISSUED,
            remaining_attempts: 0,
            remaining_retries: 0,
            expires_at: DateTime::parse_from_rfc3339("2023-10-18T10:59:50.159306Z").unwrap(),
            extra_fields: Default::default(),
        };
        let expected = OAuthResponse {
            detail: "detail".to_string(),
            challenge: Some(expected_challenge),
            mfa_required: true,
            access_token: Some("access".to_string()),
            refresh_token: Some("refresh".to_string()),
            expires_in: 100,
            extra_fields: HashMap::from([("some_other_field".to_string(), json!("extra"))]),
        };
        assert_eq!(oauth, expected)
    }

    #[test]
    fn oauth_response_deserialize_minimal() {
        let json = indoc! {
            r#"
            {
              "detail": "detail",
              "expires_in": 100
            }
            "#
        };

        let oauth: OAuthResponse = serde_json::from_str(json).expect("failed to deserialize");

        let expected = OAuthResponse {
            detail: "detail".to_string(),
            challenge: None,
            mfa_required: false,
            access_token: None,
            refresh_token: None,
            expires_in: 100,
            extra_fields: Default::default(),
        };
        assert_eq!(oauth, expected)
    }
}
