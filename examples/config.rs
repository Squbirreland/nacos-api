use nacos_api::{NacosConfig, NacosConfigApi, NacosConfigClient};
use nacos_api::model::DeployConfig;


#[tokio::main]
async fn main() {
    let client = NacosConfigClient::from(test_config_api());
    let nacos_config = test_nacos_config();
    client.listen_config(
        &nacos_config,
        |s| { println!(" perceive the configs changed to > {}", s) },
        10
    ).await;
    loop {}
}

fn test_nacos_config() -> NacosConfig {
    NacosConfig::new("http", "192.168.0.132", 8848)
}

fn test_config_api() -> NacosConfigApi {
    let dc = test_deploy_config();
    NacosConfigApi::new(dc)
}

fn test_deploy_config() -> DeployConfig {
    DeployConfig::new("test_data", "test_grep", None)
}

#[cfg(test)]
mod config_test {
    use crate::{test_nacos_config, test_deploy_config, test_config_api};
    use nacos_api::{NacosConfig, NacosConfigApi};

    #[tokio::test]
    async fn test_upload_configs() {
        let nacos: NacosConfig = test_nacos_config();
        let result = NacosConfigApi::upload_configs(
            &nacos,
            test_deploy_config(),
            "test_content  some config in here",
            None,
        ).await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_configs() {
        let nacos: NacosConfig = test_nacos_config();
        let result = test_config_api().get_configs(&nacos).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_delete_configs() {
        let nacos: NacosConfig = test_nacos_config();
        NacosConfigApi::delete_configs(&nacos, test_deploy_config()).await.unwrap();
    }
}