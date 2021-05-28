use crate::model::Dto;
use std::collections::HashMap;
use nacos_api_macro::Dto;

#[derive(Default, Dto)]
/// 注册实例的可选项
pub struct RegisterInstanceOption {
    /// 命名空间ID
    namespace_id: Option<String>,
    /// 权重
    weight: Option<f64>,
    /// 是否上线
    enabled: Option<bool>,
    /// 是否健康
    healthy: Option<bool>,
    /// 扩展信息
    metadata: Option<String>,
    /// 集群名
    cluster_name: Option<String>,
}

impl RegisterInstanceOption {
    pub fn set_namespace_id(&mut self, namespace_id: Option<String>) {
        self.namespace_id = namespace_id;
    }
    pub fn set_weight(&mut self, weight: Option<f64>) {
        self.weight = weight;
    }
    pub fn set_enabled(&mut self, enabled: Option<bool>) {
        self.enabled = enabled;
    }
    pub fn set_healthy(&mut self, healthy: Option<bool>) {
        self.healthy = healthy;
    }
    pub fn set_metadata(&mut self, metadata: Option<String>) {
        self.metadata = metadata;
    }
    pub fn set_cluster_name(&mut self, cluster_name: Option<String>) {
        self.cluster_name = cluster_name;
    }
    pub fn namespace_id(&self) -> &Option<String> {
        &self.namespace_id
    }
    pub fn weight(&self) -> Option<f64> {
        self.weight
    }
    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }
    pub fn healthy(&self) -> Option<bool> {
        self.healthy
    }
    pub fn metadata(&self) -> &Option<String> {
        &self.metadata
    }
    pub fn cluster_name(&self) -> &Option<String> {
        &self.cluster_name
    }
}

impl RegisterInstanceOption {
    pub fn new(namespace_id: Option<String>, weight: Option<f64>, enabled: Option<bool>, healthy: Option<bool>, metadata: Option<String>, cluster_name: Option<String>) -> Self {
        RegisterInstanceOption { namespace_id, weight, enabled, healthy, metadata, cluster_name }
    }
}

/// 注销实例可选项
#[derive(Default, Dto)]
pub struct RemoveInstanceOption {
    /// 集群名称
    cluster_name: Option<String>,
    /// 命名空间ID
    namespace_id: Option<String>,
}

impl RemoveInstanceOption {
    pub fn set_cluster_name(&mut self, cluster_name: Option<String>) {
        self.cluster_name = cluster_name;
    }
    pub fn set_namespace_id(&mut self, namespace_id: Option<String>) {
        self.namespace_id = namespace_id;
    }
    pub fn cluster_name(&self) -> &Option<String> {
        &self.cluster_name
    }
    pub fn namespace_id(&self) -> &Option<String> {
        &self.namespace_id
    }
}

#[derive(Default, Dto)]
/// 修改实例可选项
pub struct UpdateInstanceOption {
    /// 集群名称
    cluster_name: Option<String>,
    /// 命名空间ID
    namespace_id: Option<String>,
    /// 权重
    weight: Option<f64>,
    /// 扩展信息 JSON
    metadata: Option<String>,
    /// 是否打开流量
    enabled: Option<bool>,
}

impl UpdateInstanceOption {
    pub fn set_cluster_name(&mut self, cluster_name: Option<String>) {
        self.cluster_name = cluster_name;
    }
    pub fn set_namespace_id(&mut self, namespace_id: Option<String>) {
        self.namespace_id = namespace_id;
    }
    pub fn set_weight(&mut self, weight: Option<f64>) {
        self.weight = weight;
    }
    pub fn set_metadata(&mut self, metadata: Option<String>) {
        self.metadata = metadata;
    }
    pub fn set_enabled(&mut self, enabled: Option<bool>) {
        self.enabled = enabled;
    }
    pub fn cluster_name(&self) -> &Option<String> {
        &self.cluster_name
    }
    pub fn namespace_id(&self) -> &Option<String> {
        &self.namespace_id
    }
    pub fn weight(&self) -> Option<f64> {
        self.weight
    }
    pub fn metadata(&self) -> &Option<String> {
        &self.metadata
    }
    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }
}

#[derive(Default, Dto)]
/// 获取实例选项
pub struct GetInstanceOption {
    /// 命名空间ID
    namespace_id: Option<String>,
    /// 集群名称 多个用 , 分割
    clusters: Option<String>,
    /// 是否只返回健康实例
    healthy_only: Option<bool>,
}

impl GetInstanceOption {
    pub fn set_namespace_id(&mut self, namespace_id: Option<String>) {
        self.namespace_id = namespace_id;
    }
    pub fn set_clusters(&mut self, clusters: Option<String>) {
        self.clusters = clusters;
    }
    pub fn set_healthy_only(&mut self, healthy_only: Option<bool>) {
        self.healthy_only = healthy_only;
    }
    pub fn namespace_id(&self) -> &Option<String> {
        &self.namespace_id
    }
    pub fn clusters(&self) -> &Option<String> {
        &self.clusters
    }
    pub fn healthy_only(&self) -> Option<bool> {
        self.healthy_only
    }
}

#[derive(Debug, Default, Dto)]
pub struct PostServiceOption {
    group_name: Option<String>,
    namespace_id: Option<String>,
    protect_threshold: Option<f64>,
    metadata: Option<String>,
    selector: Option<String>,
}

impl PostServiceOption {
    pub fn set_group_name(&mut self, group_name: Option<String>) {
        self.group_name = group_name;
    }
    pub fn set_namespace_id(&mut self, namespace_id: Option<String>) {
        self.namespace_id = namespace_id;
    }
    pub fn set_protect_threshold(&mut self, protect_threshold: Option<f64>) {
        self.protect_threshold = protect_threshold;
    }
    pub fn set_metadata(&mut self, metadata: Option<String>) {
        self.metadata = metadata;
    }
    pub fn set_selector(&mut self, selector: Option<String>) {
        self.selector = selector;
    }
    pub fn group_name(&self) -> &Option<String> {
        &self.group_name
    }
    pub fn namespace_id(&self) -> &Option<String> {
        &self.namespace_id
    }
    pub fn protect_threshold(&self) -> Option<f64> {
        self.protect_threshold
    }
    pub fn metadata(&self) -> &Option<String> {
        &self.metadata
    }
    pub fn selector(&self) -> &Option<String> {
        &self.selector
    }
}

#[derive(Debug, Default, Dto)]
pub struct DeleteServiceOption {
    group_name: Option<String>,
    namespace_id: Option<String>,
}

impl DeleteServiceOption {
    pub fn set_group_name(&mut self, group_name: Option<String>) {
        self.group_name = group_name;
    }
    pub fn set_namespace_id(&mut self, namespace_id: Option<String>) {
        self.namespace_id = namespace_id;
    }
    pub fn group_name(&self) -> &Option<String> {
        &self.group_name
    }
    pub fn namespace_id(&self) -> &Option<String> {
        &self.namespace_id
    }
}

#[derive(Debug, Default, Dto)]
pub struct PutServiceOption {
    group_name: Option<String>,
    namespace_id: Option<String>,
    protect_threshold: Option<f64>,
    metadata: Option<String>,
    selector: Option<String>,
}

impl PutServiceOption {
    pub fn set_group_name(&mut self, group_name: Option<String>) {
        self.group_name = group_name;
    }
    pub fn set_namespace_id(&mut self, namespace_id: Option<String>) {
        self.namespace_id = namespace_id;
    }
    pub fn set_protect_threshold(&mut self, protect_threshold: Option<f64>) {
        self.protect_threshold = protect_threshold;
    }
    pub fn set_metadata(&mut self, metadata: Option<String>) {
        self.metadata = metadata;
    }
    pub fn set_selector(&mut self, selector: Option<String>) {
        self.selector = selector;
    }
    pub fn group_name(&self) -> &Option<String> {
        &self.group_name
    }
    pub fn namespace_id(&self) -> &Option<String> {
        &self.namespace_id
    }
    pub fn protect_threshold(&self) -> Option<f64> {
        self.protect_threshold
    }
    pub fn metadata(&self) -> &Option<String> {
        &self.metadata
    }
    pub fn selector(&self) -> &Option<String> {
        &self.selector
    }
}

#[derive(Debug, Default, Dto)]
pub struct GetServiceOption {
    group_name: Option<String>,
    namespace_id: Option<String>,
}

impl GetServiceOption {
    pub fn set_group_name(&mut self, group_name: Option<String>) {
        self.group_name = group_name;
    }
    pub fn set_namespace_id(&mut self, namespace_id: Option<String>) {
        self.namespace_id = namespace_id;
    }
    pub fn group_name(&self) -> &Option<String> {
        &self.group_name
    }
    pub fn namespace_id(&self) -> &Option<String> {
        &self.namespace_id
    }
}

#[derive(Debug, Default, Dto)]
pub struct PostConfigsOption {

}