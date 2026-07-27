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
    #[prost(string, tag = "6")]
    pub order_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct LimitOrderRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub symbol: String,
    #[prost(double, tag = "3")]
    pub quantity: f64,
    #[prost(uint32, tag = "4")]
    pub side: u32,
    #[prost(double, tag = "5")]
    pub price: f64,
    #[prost(string, tag = "6")]
    pub order_type: String,
    #[prost(double, optional, tag = "7")]
    pub tp: Option<f64>,
    #[prost(double, optional, tag = "8")]
    pub sl: Option<f64>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ModifyOrderRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub order_id: String,
    #[prost(string, tag = "3")]
    pub symbol: String,
    #[prost(double, tag = "4")]
    pub tp: f64,
    #[prost(double, tag = "5")]
    pub sl: f64,
}

#[derive(Clone, PartialEq, Message)]
pub struct CloseOrderRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub order_id: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct OrderResponse {
    #[prost(string, tag = "1")]
    pub message: String,
    #[prost(double, tag = "2")]
    pub quantity: f64,
}

#[derive(Clone, PartialEq, Message)]
pub struct LimitOrderPayload {
    #[prost(string, tag = "1")]
    pub order_id: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
    #[prost(string, tag = "3")]
    pub symbol: String,
    #[prost(double, tag = "4")]
    pub quantity: f64,
    #[prost(uint32, tag = "5")]
    pub side: u32,
    #[prost(double, tag = "8")]
    pub price: f64,
    #[prost(double, optional, tag = "9")]
    pub tp: Option<f64>,
    #[prost(double, optional, tag = "10")]
    pub sl: Option<f64>,
}

#[derive(Clone, PartialEq, Message)]
pub struct LimitOrderResult {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, tag = "2")]
    pub message: String,
    #[prost(double, optional, tag = "3")]
    pub remaining_quantity: Option<f64>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CloseOrderResponse {
    #[prost(string, tag = "1")]
    pub message: String,
    #[prost(double, tag = "2")]
    pub close_price: f64,
}

#[derive(Clone, PartialEq, Message)]
pub struct ModifyResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}
