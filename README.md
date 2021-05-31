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

#### configs

- upload config 上传配置
- get config 获取配置
- delete config 删除配置
- listen config 监听配置

### How To Use

#### register current instance

```rust
use nacos_api::{NacosClient, NacosConfig, ServerConfig};

#[tokio::main]
async fn main() {
    let nacos = NacosConfig::new("http", "192.168.0.132", 8848);
    let client = NacosClient::new(
        &nacos,
        ServerConfig::new("127.0.0.1", 8080, "test"),
    );

    client.register(&None).await;
    loop {}
}
```

#### try got other server address

```rust
pub async fn try_req_server() {
    use nacos_api::{NacosClient, NacosConfig, ServerConfig};

    let nacos = NacosConfig::new("http", "192.168.0.132", 8848);
    let client = NacosClient::new(
        &nacos,
        ServerConfig::new("127.0.0.1", 8080, "test"),
    );

    let addr = client.get_addr_simple("test").await?;
    assert!("http://127.0.0.1:8080", addr.as_str());
}
```

#### listen configs center

```rust
use nacos_api::{NacosConfigClient, NacosConfigApi, DeployConfig, NacosConfig};

#[tokio::main]
async fn main() {
    let client = NacosConfigClient::new("test_data", "test_grep", None);
    let nacos_config = NacosConfig::new("http", "192.168.0.132", 8848);
    // listen the nacos configs center
    client.listen_config(
        &nacos_config,
        |s| { println!(" perceive the configs changed to > {}", s) },
        10
    ).await;
    // make the program keep alive
    loop {}
}
```

### Declaration

    development by nacos v2.0.1