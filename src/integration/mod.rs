use crate::model::{NacosConfig, ServerConfig};
use crate::api::service::NacosServiceApi;
use crate::model::service_dto::{RegisterInstanceOption, GetInstanceOption};
use tokio::{task, time};
use tokio::time::Duration;
use std::error::Error;
use crate::model::err::NacosError;
use rand::Rng;

#[derive(Clone)]
pub struct NacosClient {
    nacos_config: NacosConfig,
    service_api: NacosServiceApi,
}

impl NacosClient {
    pub fn new(nacos_config: NacosConfig, server_config: ServerConfig)
               -> Self {
        Self {
            nacos_config,
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
    /// use tokio::net::TcpListener;
    /// use nacos_api::{NacosClient,NacosConfig, ServerConfig};
    ///
    /// // must open the port what config in ServerConfig to ack for nacos server
    /// tokio::spawn(async {
    /// let tcp_listen = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    ///     loop {
    ///         let (_, b) = tcp_listen.accept().await.unwrap();
    ///         println!(" - addr from : {:?}", b);
    ///     }
    /// });
    ///
    /// // create a client
    /// let client = NacosClient::new(
    ///     NacosConfig::new(
    ///         "http",
    ///         "139.155.225.19",
    ///         8848),
    ///     ServerConfig::new(
    ///         "127.0.0.1",
    ///         8080,
    ///         "test"),
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
        println!(" -- [debug] register success");
        let client = self.clone();
        let handle = task::spawn(hart_beat_stay(client));
        handle.await.unwrap();
    }

    /// 随机获取一个健康实例的请求地址
    /// get a random health instance`s request address .
    /// ```rust
    /// use nacos_api::{NacosClient,NacosConfig, ServerConfig};
    ///
    /// let client = NacosClient::new(
    ///     NacosConfig::new(
    ///         "http",
    ///         "139.155.225.19",
    ///         8848),
    ///     ServerConfig::new(
    ///         "127.0.0.1",
    ///         8080,
    ///         "test"),
    /// );
    /// let addr = client.get_addr_simple("test","/hi/friend").await?;
    /// assert!("http://127.0.0.1:8080/hi/friend", addr.as_str());
    /// ```
    pub async fn get_addr_simple(&self, server_name: &str, req_addr: &str) -> Result<String, Box<dyn Error>> {
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
                let req = if let Some(s) = req_addr.strip_prefix('/')
                { s.to_string() } else { req_addr.to_string() };
                format!("http://{}:{}/{}", nh.ip, nh.port, req)
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
        println!(" -- [debug] hart beat : {:?}", br);
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

#[cfg(test)]
mod tests {
    use crate::integration::NacosClient;
    use crate::model::{NacosConfig, ServerConfig};

    fn test_client() -> NacosClient {
        NacosClient::new(
            NacosConfig::new(
                "http",
                "139.155.225.19",
                8848),
            ServerConfig::new(
                "127.0.0.1",
                8080,
                "test"),
        )
    }

    #[tokio::test]
    async fn test_register() {
        use tokio::net::TcpListener;

        // must open the port what config in ServerConfig to ack for nacos server
        tokio::spawn(async {
            let tcp_listen = TcpListener::bind("0.0.0.0:8080").await.unwrap();
            loop {
                let (_, b) = tcp_listen.accept().await.unwrap();
                println!(" - addr from : {:?}", b);
            }
        });
        test_client().register(&None).await;
    }

    #[tokio::test]
    async fn test_get_addr_simple() {
        let addr = test_client().get_addr_simple("test", "").await.unwrap();
        println!(" -- > addr : {}", addr);
    }
}