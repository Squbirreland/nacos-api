use std::error::Error;
use std::fmt::{Display, Formatter};

/// 自定 Nacos 返回的错误对象
#[derive(Debug)]
pub struct NacosError {
    reason: String,
}

impl NacosError {
    pub fn throw(reason: &str) -> Self {
        Self {
            reason: reason.to_string(),
        }
    }
}

impl Display for NacosError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, " - nacos server return err - ")
    }
}

impl Error for NacosError {}