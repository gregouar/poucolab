use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTestResponse {
    pub now: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTestDataResponse {
    pub processed_at: DateTime<Utc>,
}
