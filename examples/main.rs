use nacos_api::{NacosClient, NacosConfig, ServerConfig};

#[tokio::main]
async fn main() {
    let client = NacosClient::new(
        NacosConfig::new(
            "http",
            "192.168.0.132",
            8848,
        ),
        ServerConfig::new(
            "127.0.0.1",
            8080,
            "test",
        ),
    );
    client.register(&None).await;
    loop {}
}