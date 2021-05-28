use crate::model::{NacosConfig, ServerConfig};
use crate::api::service::NacosServiceApi;
use crate::model::service_dto::{RegisterInstanceOption, GetInstanceOption};
use tokio::{task, time};
use tokio::time::Duration;
use std::error::Error;
use crate::model::err::NacosError;
use rand::Rng;

/// NacosClient 是主要的nacos服务调用结构 ,
/// NacosClient is the primary struct to call nacos server .
#[derive(Clone)]
pub struct NacosClient {
    nacos_config: NacosConfig,
    service_api: NacosServiceApi,
}

impl NacosClient {
    pub fn new(nacos_config: &NacosConfig, server_config: ServerConfig) -> Self {
        Self {
            nacos_config: nacos_config.clone(),
            service_api: NacosServiceApi::new(server_config),
        }
    }
    pub fn nacos_config(&self) -> &NacosConfig {
        &self.nacos_config
    }
    pub fn service_api(&self) -> &NacosServiceApi {
        &self.service_api
    }
    pub fn nacos_config_mut(&mut self) -> &mut NacosConfig {
        &mut self.nacos_config
    }
    pub fn service_api_mut(&mut self) -> &mut NacosServiceApi {
        &mut self.service_api
    }
}

impl NacosClient {
    /// 注册当前实例并自动发送/回应心跳
    /// register current instance and send/ack hart beat.
    /// ```rust
    /// use nacos_api::{NacosClient,NacosConfig, ServerConfig};
    ///
    /// // create a client
    /// let nacos = NacosConfig::new("http", "192.168.0.132", 8848);
    /// let client = NacosClient::new(
    ///     &nacos,
    ///     ServerConfig::new("127.0.0.1", 8080, "test"),
    /// );
    ///
    /// // register current instance to nacos
    /// // and new thread in background send and ack hart beat
    /// client.register(&None).await;
    /// ```
    pub async fn register(&self, option: &Option<RegisterInstanceOption>) {
        if let Err(e) = self.service_api
            .register_instance(self.nacos_config(), option)
            .await { panic!("{:?}", e) };
        println!(" -- [info] nacos register success");
        let client = self.clone();
        task::spawn(hart_beat_stay(client));
    }

    /// 随机获取一个健康实例的请求地址
    /// get a random health instance`s request address .
    /// ```rust
    /// use nacos_api::{NacosClient,NacosConfig, ServerConfig};
    ///
    /// let nacos = NacosConfig::new("http", "192.168.0.132", 8848);
    /// let client = NacosClient::new(
    ///     &nacos,
    ///     ServerConfig::new("127.0.0.1", 8080, "test"),
    /// );
    ///
    /// let addr = client.get_addr_simple("test").await?;
    /// assert!("http://127.0.0.1:8080", addr.as_str());
    /// ```
    pub async fn get_addr_simple(&self, server_name: &str) -> Result<String, Box<dyn Error>> {
        let mut option = GetInstanceOption::default();
        option.set_healthy_only(Some(true));
        let list = NacosServiceApi::get_instance_list(
            self.nacos_config(),
            server_name,
            &Some(option)).await?;
        let addr = match list.hosts {
            None => {
                return Err(Box::new(NacosError::throw(" -- err : server have not instance ")));
            }
            Some(v) => {
                if v.is_empty() { return Err(Box::new(NacosError::throw(" -- err : server have not instance "))); }
                let r = rand::thread_rng().gen_range(0..v.len());
                let nh = &v[r];
                format!("http://{}:{}", nh.ip, nh.port)
            }
        };
        Ok(addr)
    }
}

async fn hart_beat_stay(client: NacosClient) {
    let mut beat: Option<String> = None;
    'hb: loop {
        let br = match &beat {
            None => {
                client.service_api.hart_beat(client.nacos_config()).await
            }
            Some(bt) => {
                client.service_api.hart_beat_weight(client.nacos_config(), &bt).await
            }
        };
        match br {
            Ok(nb) => {
                let config = client.service_api.config();
                //如果重拍 获取信息
                if !nb.light_beat_enabled {
                    let bt = match NacosServiceApi::get_instance_str(
                        &client.nacos_config,
                        config.server_name(),
                        config.server_ip(),
                        config.server_port(),
                        &None,
                    ).await {
                        Ok(beat) => { beat }
                        Err(e) => {
                            println!(" -- hart beat query info err : {:?}", e);
                            break 'hb;
                        }
                    };
                    beat = Some(bt);
                }
                // delay
                let delay = if nb.client_beat_interval > 2
                { nb.client_beat_interval - 2 } else { nb.client_beat_interval };
                time::sleep(Duration::from_millis(delay)).await;
            }
            Err(e) => {
                println!(" -- hart beat err : {:?}", e);
                break 'hb;
            }
        }
    }
}

