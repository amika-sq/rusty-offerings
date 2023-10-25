use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatusData {
    ///  Current status of Order that's being executed
    pub order_status: String,
}
