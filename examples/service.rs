use nacos_api::{NacosClient, NacosConfig, ServerConfig};

#[tokio::main]
async fn main() {
    let client = test_client();
    client.register(&None).await;
    loop {}
}

fn test_nacos_config() -> NacosConfig {
    NacosConfig::new("http", "192.168.0.132", 8848)
}

fn test_client() -> NacosClient {
    NacosClient::new(
        &test_nacos_config(),
        ServerConfig::new(
            "127.0.0.1",
            8080,
            "test",
        ),
    )
}

#[cfg(test)]
mod server_test {
    use crate::{test_client};
    use nacos_api::NacosServiceApi;

    #[tokio::test]
    async fn test_get_addr_simple() {
        let addr = test_client().get_addr_simple("test").await.unwrap();
        println!(" -- > addr : {}", addr);
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance = NacosServiceApi::get_instance(
            test_client().nacos_config(),
            "test", "127.0.0.1",
            8080,
            &None,
        ).await;
        println!(" -- > instance : {:?}", instance);
    }

    #[tokio::test]
    async fn test_get_server() {
        let server = NacosServiceApi::get_server(
            test_client().nacos_config(),
            "test",
            &None,
        ).await;
        println!(" -- > server : {:?}", server);
    }

    #[tokio::test]
    async fn test_get_server_list() {
        let server_list = NacosServiceApi::get_server_list(
            test_client().nacos_config(),
            1,
            10,
            &None,
        ).await;
        println!(" -- > server_list : {:?}", server_list);
    }

    #[tokio::test]
    async fn test_get_operator_servers() {
        let operator_servers = NacosServiceApi::get_operator_servers(
            test_client().nacos_config()
        ).await;
        println!(" -- > operator_servers : {:?}", operator_servers);
    }

    #[tokio::test]
    async fn test_get_operator_metrics() {
        let operator_metrics = NacosServiceApi::get_operator_metrics(
            test_client().nacos_config()
        ).await;
        println!(" -- > operator_metrics : {:?}", operator_metrics);
    }
}

