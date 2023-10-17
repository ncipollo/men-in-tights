use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ChallengeType {
    EMAIL,
    SMS
}