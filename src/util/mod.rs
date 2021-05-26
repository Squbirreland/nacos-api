use std::collections::HashMap;
use reqwest::{Response, RequestBuilder, Client};
use std::error::Error;
use crate::model::Dto;
use crate::model;
use crate::model::err::NacosError;

pub(crate) async fn send<T, F>(mut map: HashMap::<String, String>, option: &Option<T>, fnc: F)
                               -> Result<Response, Box<dyn Error>>
    where T: Dto,
          F: Fn(Client) -> RequestBuilder
{
    model::catch_mapping(&mut map, option);
    let client = Client::new();
    let resp = fnc(client).query(&map).send().await?;
    Ok(resp)
}

pub(crate) async fn send_and_ok<T, F>(map: HashMap::<String, String>, option: &Option<T>, fnc: F)
                                      -> Result<(), Box<dyn Error>>
    where T: Dto,
          F: Fn(Client) -> RequestBuilder
{
    let resp = send(map, option, fnc).await?;
    let result = resp.text().await?;
    if result.ne("ok") { return Err(Box::from(NacosError::throw(&result))); }
    Ok(())
}