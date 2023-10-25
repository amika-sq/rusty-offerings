use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Close {
    /// An explanation of why the exchange is being closed/completed
    pub reason: String,
}
