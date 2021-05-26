## NACOS API

### Supported

#### service

- register instance 注册实例
- remove instance 注销实例
- update instance 修改实例
- get instance list 查询实例列表
- get instance 查询实例详情
- hart beat 心跳感应
- create service 创建服务
- delete service 删除服务
- update service 修改服务
- get service information 查询服务详情
- get service list 查询服务列表
- get operator metrics 查询系统运载
- get operator servers 查询服务详情

### How To Use

```rust
pub async fn init() {
    let client = NacosClient::new(
        NacosConfig::new(
            "http",
            "139.155.225.19",
            8848),
        ServerConfig::new(
            "127.0.0.1",
            8080,
            "test"),
    );
    client.register(&None).await;
}

pub async fn try_req_server() {
    use nacos_api::{NacosClient,NacosConfig, ServerConfig};

    let client = NacosClient::new(
        NacosConfig::new(
            "http",
            "139.155.225.19",
            8848),
        ServerConfig::new(
            "127.0.0.1",
            8080,
            "test"),
    );
    let addr = client.get_addr_simple("test", "/hi/friend").await?;
    assert!("http://127.0.0.1:8080/hi/friend", addr.as_str());
}
```
    