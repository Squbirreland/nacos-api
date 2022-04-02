use crate::{NacosConfigApi, NacosConfig, DeployConfig};
use tokio::task;
use tokio::time;
use std::time::Duration;

/// NacosClient 是主要的nacos配置中心调用结构 ,
/// NacosClient is the primary struct to call nacos configs center .
#[derive(Clone)]
pub struct NacosConfigClient {
    config_api: NacosConfigApi,
}

impl NacosConfigClient {
    pub fn new(data_id: &str, group: &str, tenant: Option<String>) -> Self
    { Self { config_api: NacosConfigApi::new(DeployConfig::new(data_id, group, tenant)) } }
    pub fn from(config_api: NacosConfigApi) -> Self { Self { config_api } }
    pub fn config_api(&self) -> &NacosConfigApi { &self.config_api }
}

impl NacosConfigClient {
    /// 后台线程持续监听配置 如果改变 则将改变后的配置返回给[func] ,
    /// keep listen configs in background and return changed configs to [func] .
    /// ```rust
    /// use nacos_api::{NacosConfigClient, NacosConfigApi, DeployConfig, NacosConfig};
    ///
    /// let client = NacosConfigClient::new("test_data", "test_grep", None);
    /// let nacos_config = NacosConfig::new("http", "139.155.225.19", 8848);
    /// // listen the nacos configs center
    /// client.listen_config(
    ///     &nacos_config,
    ///     |s| { println!(" perceive the configs changed to > {}", s) },
    ///     10
    /// ).await;
    /// // make the program keep alive
    /// loop {}
    /// ```
    pub async fn listen_config<F>(&self, nacos_config: &NacosConfig, func: F, interval_secs: u64)
        where F: Fn(&String) + Send + 'static
    {
        task::spawn(listen(
            self.config_api.clone(),
            nacos_config.clone(),
            func,
            interval_secs,
        )).await.unwrap()
    }
}

async fn listen<F>(config_api: NacosConfigApi, nacos_config: NacosConfig, func: F, interval_secs: u64)
    where F: Fn(&String)
{
    let prev_conf = match config_api.get_configs(&nacos_config).await {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("config_api.get_configs error: {}, nacos_config: {:?}", err, nacos_config);
            "".to_owned()
        }
    };
    let mut prev_conf_md5 = format!("{:x}", md5::compute(prev_conf));
    println!(" -- [debug] starting listen configs");
    loop {
        time::sleep(Duration::from_secs(interval_secs)).await;
        let current_conf = match config_api.get_configs(&nacos_config).await {
            Ok(conf) => conf,
            Err(err) => {
                eprintln!("config_api.get_configs error: {}, nacos_config: {:?}", err, nacos_config);
                "".to_owned()
            }
        };
        let current_conf_md5 = format!("{:x}", md5::compute(&current_conf));
        if prev_conf_md5.ne(&current_conf_md5) {
            func(&current_conf);
            prev_conf_md5 = current_conf_md5;
        }
    }
}