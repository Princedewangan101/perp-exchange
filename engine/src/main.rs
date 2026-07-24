use futures::StreamExt;

mod order_book;

#[tokio::main]
async fn main() {
    engine().await;
}

async fn engine() {
    let nats_client = async_nats::connect("127.0.0.1:4222").await.unwrap();

    // let btc_perp_market = Market::new("BTC".to_string());

    let mut subscriber = nats_client.subscribe("order.*").await.unwrap();

    while let Some(message) = subscriber.next().await {
        let subject = message.subject.as_str();

        let payload_string = std::str::from_utf8(&message.payload);

        match subject {
            "order.limit" => {

            }
            "order.market" => {
                println!("\n[INFO] subject: {}, payload: {:#?}", subject, payload_string)
            }
            "order.modify" => {
                println!("\n[INFO] subject: {}, payload: {:#?}", subject, payload_string)
            }
            "order.close" => {
                println!("\n[INFO] subject: {}, payload: {:#?}", subject, payload_string)
            }
            "order.close.all" => {
                println!("\n[INFO] subject: {}, payload: {:#?}", subject, payload_string)
            }
            _ => {
                eprintln!("\n[ERROR] subject not matched, subject: {}", subject)
            }
        }
    }
}