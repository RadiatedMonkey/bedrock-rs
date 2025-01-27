use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let (mut server, mut handle) = bedrockrs::server::ServerBuilder::new()
        .name("Chick-Topia")
        .sub_name("bedrock-rs")
        .build()
        .await;

    println!("Server started");

    tokio::spawn(async move {
        println!("Server starting");
        server.run().await
    });

    sleep(Duration::from_secs(5)).await;

    handle.shutdown_graceful().await;

    println!("server shutdown");
}
