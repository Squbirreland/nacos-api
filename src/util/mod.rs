use std::collections::HashMap;
use reqwest::{Response, RequestBuilder, Client};
use std::error::Error;
use crate::model::{self, Dto};
use crate::model::err::NacosError;
use lazy_static::lazy_static;
use std::time::Duration;

lazy_static! {
    pub static ref CLIENT: Client = Client::new();
}

pub(crate) async fn query<F>(map: &HashMap<String, String>, func: F)
                             -> Result<Response, Box<dyn Error>>
    where F: Fn(&Client) -> RequestBuilder
{
    let resp = func(&CLIENT).query(map).timeout(Duration::from_secs(10)).send().await?;
    Ok(resp)
}

pub(crate) async fn query_resp<T, F>(mut map: HashMap<String, String>, option: &Option<T>, func: F)
                                     -> Result<Response, Box<dyn Error>>
    where T: Dto,
          F: Fn(&Client) -> RequestBuilder
{
    model::catch_mapping(&mut map, option);
    let resp = query(&map, func).await?;
    Ok(resp)
}

pub(crate) async fn query_and_ok<T, F>(map: HashMap<String, String>, option: &Option<T>, func: F)
                                       -> Result<(), Box<dyn Error>>
    where T: Dto,
          F: Fn(&Client) -> RequestBuilder
{
    resp_assert(query_resp(map, option, func).await?, "ok").await
}

pub(crate) async fn resp_assert(resp: Response, assert: &str) -> Result<(), Box<dyn Error>> {
    let result = resp.text().await?;
    if result.ne(assert) { return Err(Box::from(NacosError::throw(&result))); }
    Ok(())
}