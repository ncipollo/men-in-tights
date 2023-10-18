use std::collections::HashMap;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Challenge {
    pub id: String,
    pub user: String,
    #[serde(rename = "type")]
    pub challenge_type: ChallengeType,
    pub status: ChallengeStatus,
    #[serde(default)]
    pub remaining_attempts: i32,
    #[serde(default)]
    pub remaining_retries: i32,
    pub expires_at: DateTime<FixedOffset>,
    #[serde(flatten)]
    pub extra_fields: HashMap<String, Value>
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChallengeType {
    EMAIL,
    SMS,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChallengeStatus {
    ISSUED,
    VALIDATED,
    FAILED
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use chrono::{DateTime, Utc};
    use indoc::indoc;
    use serde_json::json;
    use crate::session::challenge::{Challenge, ChallengeStatus, ChallengeType};

    #[test]
    fn challenge_deserialize() {
        let json = indoc! {
            r#"
            {
              "id": "id",
              "user": "user",
              "type": "sms",
              "status": "issued",
              "remaining_attempts": 3,
              "remaining_retries": 2,
              "expires_at": "2023-10-18T10:59:50.159306Z",
              "some_other_field": "extra"
            }
            "#
        };
        let challenge: Challenge = serde_json::from_str(json).expect("failed to deserialize");
        let expected = Challenge{
            id: "id".to_string(),
            user: "user".to_string(),
            challenge_type: ChallengeType::SMS,
            status: ChallengeStatus::ISSUED,
            remaining_attempts: 3,
            remaining_retries: 2,
            expires_at: DateTime::parse_from_rfc3339("2023-10-18T10:59:50.159306Z").unwrap(),
            extra_fields: HashMap::from([("some_other_field".to_string(), json!("extra"))])
        };
        assert_eq!(challenge, expected);
    }
}