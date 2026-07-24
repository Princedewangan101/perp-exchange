use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct OrderRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub symbol: String,
    #[prost(double, tag = "3")]
    pub quantity: f64,
    #[prost(uint32, tag = "4")]
    pub side: u32,
    #[prost(string, tag = "5")]
    pub order_type: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct OrderResponse {
    #[prost(string, tag = "1")]
    pub message: String,
    #[prost(double, tag = "2")]
    pub quantity: f64,
}