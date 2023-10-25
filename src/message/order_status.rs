use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatus {
    ///  Current status of Order that's being executed
    pub order_status: String,
}
