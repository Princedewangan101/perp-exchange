use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct OrderRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub symbol: String,
    #[prost(uint32, tag = "3")]
    pub quantity: u32,
    #[prost(uint32, tag = "4")]
    pub side: u32,
    #[prost(string, tag = "5")]
    pub order_type: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct LimitOrderRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub symbol: String,
    #[prost(uint32, tag = "3")]
    pub quantity: u32,
    #[prost(uint32, tag = "4")]
    pub side: u32,
    #[prost(uint64, tag = "5")]
    pub price: u64,
    #[prost(string, tag = "6")]
    pub order_type: String,
    #[prost(uint64, optional, tag = "7")]
    pub tp: Option<u64>,
    #[prost(uint64, optional, tag = "8")]
    pub sl: Option<u64>,
}

#[derive(Clone, PartialEq, Message)]
pub struct OrderResponse {
    #[prost(string, tag = "1")]
    pub message: String,
    #[prost(uint32, tag = "2")]
    pub quantity: u32,
}
