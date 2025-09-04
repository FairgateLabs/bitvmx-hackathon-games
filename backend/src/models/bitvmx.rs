use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct P2PAddress {
    pub address: String,
    pub peer_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct OperatorKeys {
    pub pub_key: String,
    pub funding_key: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AggregatedKeySubmission {
    pub uuid: String,
    pub p2p_addresses: Vec<P2PAddress>,
    pub operator_keys: Option<Vec<String>>,
    pub leader_idx: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, ToSchema)]
#[ts(export)]
pub struct AggregatedKey {
    pub uuid: String,
    pub aggregated_key: String,
}
