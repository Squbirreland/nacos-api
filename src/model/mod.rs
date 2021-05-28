use std::collections::HashMap;

pub mod err;
pub mod service_dto;
pub mod service_vo;

/// Dto接口 请求参数结构体实现该特征
pub trait Dto {
    /// 该方法用于将 dto中的属性装填入map中 也就是定义名称与值映射
    fn mapping(&self, map: &mut HashMap<String, String>);
}

pub(crate) fn catch_mapping<T: Dto>(map: &mut HashMap<String, String>, option: &Option<T>) {
    if let Some(s) = option { s.mapping(map); }
}

#[derive(Clone)]
pub struct NacosConfig {
    scheme: String,
    nacos_ip: String,
    nacos_port: u32,
}

impl Default for NacosConfig {
    fn default() -> Self {
        Self {
            scheme: "http".to_string(),
            nacos_ip: "127.0.0.1".to_string(),
            nacos_port: 8848,
        }
    }
}

impl NacosConfig {
    pub fn new(scheme: &str, nacos_ip: &str, nacos_port: u32) -> Self {
        Self {
            scheme: scheme.to_string(),
            nacos_ip: nacos_ip.to_string(),
            nacos_port,
        }
    }

    pub fn exchange(&mut self, ex: Self) -> Self {
        let prev = self.clone();
        self.scheme = ex.scheme;
        self.nacos_ip = ex.nacos_ip;
        self.nacos_port = ex.nacos_port;
        prev
    }

    pub fn addr(&self, target: &str) -> String {
        let sub_path = if target.starts_with('/')
        { target.to_string() } else { format!("/{}", target) };
        format!(
            "{}://{}:{}/nacos{}",
            self.scheme, self.nacos_ip, self.nacos_port, sub_path
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct ServerConfig {
    server_ip: String,
    server_port: u16,
    server_name: String,
    ephemeral: bool,
    group_name: Option<String>,
}

impl ServerConfig {
    pub fn set_server_ip(&mut self, server_ip: String) {
        self.server_ip = server_ip;
    }
    pub fn set_server_port(&mut self, server_port: u16) {
        self.server_port = server_port;
    }
    pub fn set_server_name(&mut self, server_name: String) {
        self.server_name = server_name;
    }
    pub fn set_ephemeral(&mut self, ephemeral: bool) {
        self.ephemeral = ephemeral;
    }
    pub fn set_group_name(&mut self, group_name: Option<String>) {
        self.group_name = group_name;
    }
}

impl ServerConfig {
    pub fn server_ip(&self) -> &str { &self.server_ip }
    pub fn server_port(&self) -> u16 {
        self.server_port
    }
    pub fn server_name(&self) -> &str {
        &self.server_name
    }
    pub fn ephemeral(&self) -> bool {
        self.ephemeral
    }
    pub fn group_name(&self) -> &Option<String> {
        &self.group_name
    }
}

impl ServerConfig {
    pub fn new(server_ip: &str, server_port: u16, server_name: &str) -> Self {
        Self {
            server_ip: server_ip.to_string(),
            server_port,
            server_name: server_name.to_string(),
            ephemeral: false,
            group_name: None,
        }
    }

    pub(crate) fn init_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::new();
        map.insert("ip".to_string(), self.server_ip().to_string());
        map.insert("port".to_string(), self.server_port().to_string());
        map.insert("serviceName".to_string(), self.server_name().to_string());
        if self.ephemeral {
            map.insert("ephemeral".to_string(), true.to_string());
        }
        if let Some(s) = &self.group_name {
            map.insert("groupName".to_string(), s.to_string());
        }
        map
    }
}

#[derive(Default, Debug, Clone)]
pub struct DeployConfig {
    data_id: String,
    group: String,
    tenant: Option<String>,
}

impl DeployConfig {
    pub fn new(data_id: &str, group: &str, tenant: Option<String>) -> Self {
        Self {
            data_id: data_id.to_string(),
            group: group.to_string(),
            tenant,
        }
    }

    pub fn init_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::new();
        map.insert("dataId".to_string(), self.data_id.clone());
        map.insert("group".to_string(), self.group.clone());
        if let Some(tenant) = &self.tenant {
            map.insert("tenant".to_string(), tenant.clone());
        };
        map
    }

    pub fn data_id(&self) -> &str {
        &self.data_id
    }
    pub fn group(&self) -> &str {
        &self.group
    }
    pub fn tenant(&self) -> &Option<String> {
        &self.tenant
    }
}
