use crate::{NacosConfig, util};
use crate::model::DeployConfig;
use std::error::Error;

const GET_CONFIGS: &str = "/v1/cs/configs";
const POST_CONFIGS: &str = "/v1/cs/configs";
const DELETE_CONFIGS: &str = "/v1/cs/configs";

#[derive(Clone)]
pub struct NacosConfigApi {
    deploy_config: DeployConfig,
}

impl NacosConfigApi {
    pub fn new(config: DeployConfig) -> Self {
        Self {
            deploy_config: config,
        }
    }
    pub fn deploy_config(&self) -> &DeployConfig {
        &self.deploy_config
    }
}

impl NacosConfigApi {
    pub async fn get_configs(&self, nacos: &NacosConfig)
                             -> Result<String, Box<dyn Error>> {
        let map = self.deploy_config.init_map();
        let resp = util::query(&map, |c| c.get(nacos.addr(GET_CONFIGS))).await?;
        let result = resp.text().await?;
        Ok(result)
    }

    pub async fn upload_configs(nacos: &NacosConfig, config: DeployConfig, content: &str, types: Option<String>)
                                -> Result<(), Box<dyn Error>> {
        let mut map = config.init_map();
        map.insert("content".to_string(), content.to_string());
        if let Some(t) = types { map.insert("type".to_string(), t); }
        let resp = util::query(&map, |c| c.post(nacos.addr(POST_CONFIGS))).await?;
        util::resp_assert(resp, "true").await
    }

    pub async fn delete_configs(nacos: &NacosConfig, config: DeployConfig)
                                -> Result<(), Box<dyn Error>> {
        let map = config.init_map();
        let resp = util::query(&map, |c| c.delete(nacos.addr(DELETE_CONFIGS))).await?;
        util::resp_assert(resp, "true").await
    }
}