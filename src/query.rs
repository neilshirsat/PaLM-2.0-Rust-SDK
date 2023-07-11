use serde::de::DeserializeOwned;

pub struct Query {
    
    #[allow(dead_code)]
    pub(super) request_builder: reqwest::RequestBuilder

}

#[allow(dead_code)]
pub struct Response<T: DeserializeOwned> {

    pub headers: http::HeaderMap,

    pub value: T

}

#[allow(dead_code)]
pub async fn execute<T: DeserializeOwned>( query: Query ) -> Result<Response<T>, reqwest::Error> {
    
    let response = query
        .request_builder
        .send()
        .await?;

    Ok(Response {
        headers: response.headers().clone(),
        value: serde_json::from_str(&response.text().await?).unwrap()
    })

}

#[allow(dead_code)]
pub async fn execute_raw( query: Query ) -> Result<Response<String>, reqwest::Error> {
    
    let response = query
        .request_builder
        .send()
        .await?;

    Ok(Response {
        headers: response.headers().clone(),
        value: response.text().await?
    })

}