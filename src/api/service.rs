use crate::model::{NacosConfig, ServerConfig};
use std::error::Error;
use crate::model::service_dto::{RegisterInstanceOption, RemoveInstanceOption, UpdateInstanceOption, GetInstanceOption, PostServiceOption, DeleteServiceOption, PutServiceOption, GetServiceOption};
use std::collections::HashMap;
use crate::model::service_vo::{NacosServerView, NacosHost, NacosBeat, NacosServiceInfo, NacosServiceList, NacosMetrics, NacosServerSimpleView};
use crate::util::{self, CLIENT};

const POST_INSTANCE: &str = "/v1/ns/instance";
const DELETE_INSTANCE: &str = "/v1/ns/instance";
const PUT_INSTANCE: &str = "/v1/ns/instance";
const GET_INSTANCE_LIST: &str = "/v1/ns/instance/list";
const GET_INSTANCE: &str = "/v1/ns/instance";
const PUT_INSTANCE_BEAT: &str = "/v1/ns/instance/beat";

const POST_SERVICE: &str = "/v1/ns/service";
const DELETE_SERVICE: &str = "/v1/ns/service";
const PUT_SERVICE: &str = "/v1/ns/service";
const GET_SERVICE: &str = "/v1/ns/service";
const GET_SERVICE_LIST: &str = "/v1/ns/service/list";

const GET_OPERATOR_METRICS: &str = "/v1/ns/operator/metrics";
const GET_OPERATOR_SERVERS: &str = "/v1/ns/operator/servers";

/// NacosServiceApi 是nacos的服务相关api封装
/// NacosServiceApi is nacos api encapsulation of services .
#[derive(Clone)]
pub struct NacosServiceApi {
    server_config: ServerConfig,
}

impl NacosServiceApi {
    pub fn new(config: ServerConfig) -> Self { Self { server_config: config } }
    pub fn config(&self) -> &ServerConfig { &self.server_config }
    pub fn config_mut(&mut self) -> &mut ServerConfig { &mut self.server_config }
}

impl NacosServiceApi {
    /// register a new instance .
    pub async fn register_instance(&self, nacos_config: &NacosConfig, option: &Option<RegisterInstanceOption>)
                                   -> Result<(), Box<dyn Error>> {
        let map = self.server_config.init_map();
        util::query_and_ok(map, option, |c| c.post(nacos_config.addr(POST_INSTANCE))).await
    }

    /// remove current instance .
    pub async fn remove_instance(&self, nacos_config: &NacosConfig, option: &Option<RemoveInstanceOption>)
                                 -> Result<(), Box<dyn Error>> {
        let map = self.server_config.init_map();
        util::query_and_ok(map, option, |c| c.delete(nacos_config.addr(DELETE_INSTANCE))).await
    }

    /// update current instance .
    pub async fn update_instance(&self, nacos_config: &NacosConfig, option: &Option<UpdateInstanceOption>)
                                 -> Result<(), Box<dyn Error>> {
        let map = self.server_config.init_map();
        util::query_and_ok(map, option, |c| c.put(nacos_config.addr(PUT_INSTANCE))).await
    }

    /// get instance list .
    pub async fn get_instance_list(nacos_config: &NacosConfig, service_name: &str, option: &Option<GetInstanceOption>)
                                   -> Result<NacosServerView, Box<dyn Error>> {
        let mut map = HashMap::<String, String>::new();
        map.insert("serviceName".to_string(), service_name.to_string());
        let resp = util::query_resp(map, option, |c|
            c.get(nacos_config.addr(GET_INSTANCE_LIST))).await?;
        let result = resp.json::<NacosServerView>().await?;
        Ok(result)
    }

    /// get instance .
    pub async fn get_instance(nacos_config: &NacosConfig,
                              service_name: &str,
                              service_ip: &str,
                              service_port: u16,
                              option: &Option<GetInstanceOption>,
    ) -> Result<NacosHost, Box<dyn Error>> {
        let s = Self::get_instance_str(
            nacos_config,
            service_name,
            service_ip,
            service_port,
            option).await?;
        let result = serde_json::from_str::<NacosHost>(&s)?;
        Ok(result)
    }

    /// get instance return not serialized json
    pub async fn get_instance_str(nacos_config: &NacosConfig,
                                  service_name: &str,
                                  service_ip: &str,
                                  service_port: u16,
                                  option: &Option<GetInstanceOption>, )
                                  -> Result<String, Box<dyn Error>> {
        let mut map = HashMap::<String, String>::new();
        map.insert("serviceName".to_string(), service_name.to_string());
        map.insert("ip".to_string(), service_ip.to_string());
        map.insert("port".to_string(), service_port.to_string());
        let resp = util::query_resp(map, option, |c|
            c.get(nacos_config.addr(GET_INSTANCE))).await?;
        let result = resp.text().await?;
        Ok(result)
    }

    /// send hart beat
    pub async fn hart_beat(&self, nacos_config: &NacosConfig)
                           -> Result<NacosBeat, Box<dyn Error + Send + Sync>> {
        let resp = CLIENT
            .put(nacos_config.addr(PUT_INSTANCE_BEAT))
            .query(&self.server_config.init_map()).send().await?;
        let result = resp.json::<NacosBeat>().await?;
        Ok(result)
    }

    /// send hart beat with beat info
    pub async fn hart_beat_weight(&self, nacos_config: &NacosConfig, beat: &str)
                                  -> Result<NacosBeat, Box<dyn Error + Send + Sync>> {
        let mut map = self.server_config.init_map();
        map.insert("beat".to_string(), beat.to_string());
        let resp = CLIENT
            .put(nacos_config.addr(PUT_INSTANCE_BEAT))
            .query(&map).send().await?;
        let result = resp.json::<NacosBeat>().await?;
        Ok(result)
    }

    /// create a new server .
    pub async fn create_server(nacos_config: &NacosConfig, service_name: &str, option: &Option<PostServiceOption>)
                               -> Result<(), Box<dyn Error>> {
        let mut map = HashMap::<String, String>::new();
        map.insert("serviceName".to_string(), service_name.to_string());
        util::query_and_ok(map, option, |c| c.post(nacos_config.addr(POST_SERVICE))).await
    }

    /// delete a server by server name and option .
    pub async fn delete_server(nacos_config: &NacosConfig, service_name: &str, option: &Option<DeleteServiceOption>)
                               -> Result<(), Box<dyn Error>> {
        let mut map = HashMap::<String, String>::new();
        map.insert("serviceName".to_string(), service_name.to_string());
        util::query_and_ok(map, option, |c| c.delete(nacos_config.addr(DELETE_SERVICE))).await
    }

    /// update a server by server name and option .
    pub async fn update_server(nacos_config: &NacosConfig, service_name: &str, option: &Option<PutServiceOption>)
                               -> Result<(), Box<dyn Error>> {
        let mut map = HashMap::<String, String>::new();
        map.insert("serviceName".to_string(), service_name.to_string());
        map.insert("protectThreshold".to_string(), "0".to_string());
        util::query_and_ok(map, option, |c| c.put(nacos_config.addr(PUT_SERVICE))).await
    }

    /// get a server by server name and option
    pub async fn get_server(nacos_config: &NacosConfig, service_name: &str, option: &Option<GetServiceOption>)
                            -> Result<NacosServiceInfo, Box<dyn Error>> {
        let mut map = HashMap::<String, String>::new();
        map.insert("serviceName".to_string(), service_name.to_string());
        let resp = util::query_resp(map, option, |c| c.get(nacos_config.addr(GET_SERVICE))).await?;
        let result = resp.json::<NacosServiceInfo>().await?;
        Ok(result)
    }

    /// get a server list by page limit
    pub async fn get_server_list(nacos_config: &NacosConfig, page_no: i32, page_size: i32, option: &Option<GetServiceOption>)
                                 -> Result<NacosServiceList, Box<dyn Error>> {
        let mut map = HashMap::<String, String>::new();
        map.insert("pageNo".to_string(), page_no.to_string());
        map.insert("pageSize".to_string(), page_size.to_string());
        let resp = util::query_resp(map, option, |c| c.get(nacos_config.addr(GET_SERVICE_LIST))).await?;
        let result = resp.json::<NacosServiceList>().await?;
        Ok(result)
    }

    /// get current nacos cluster running state .
    pub async fn get_operator_metrics(nacos_config: &NacosConfig)
                                      -> Result<NacosMetrics, Box<dyn Error>> {
        let resp = reqwest::get(nacos_config.addr(GET_OPERATOR_METRICS)).await?;
        let result = resp.json::<NacosMetrics>().await?;
        Ok(result)
    }

    /// get current nacos cluster server info .
    /// ```rust
    /// use nacos_api::{NacosServiceApi, NacosConfig, NacosClient, ServerConfig};
    ///
    /// let nacos_config = NacosConfig::new("http","192.168.0.132",8848);
    /// //call by nacos_config
    /// let result = NacosServiceApi::get_operator_servers(&nacos_config).await;
    ///
    /// let client = NacosClient::new(&nacos_config,ServerConfig::new("127.0.0.1",8080,"test"));
    /// //call by client
    /// let result = NacosServiceApi::get_operator_servers(client.nacos_config()).await;
    /// ```
    pub async fn get_operator_servers(nacos_config: &NacosConfig)
                                      -> Result<NacosServerSimpleView, Box<dyn Error>> {
        let resp = reqwest::get(nacos_config.addr(GET_OPERATOR_SERVERS)).await?;
        let result = resp.json::<NacosServerSimpleView>().await?;
        Ok(result)
    }
}

