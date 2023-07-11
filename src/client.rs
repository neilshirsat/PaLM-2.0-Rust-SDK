use crate::{default, resource::Resource, query_builder::QueryBuilder};
use serde::Serialize;
use std::collections::HashMap;

pub trait IClient {
    
    // fn new(
    //     api_token: fn(Resource) -> String,
    //     url_generation_fn: Option<
    //         fn(
    //             resource: Resource,
    //             method: http::Method,
    //             api_token: String,
    //             path_parameters: HashMap<String, String>,
    //             query_parameters: HashMap<String, String>,
    //         ) -> String,
    //     >,
    //     operation_mapping: Option<fn(resource: Resource) -> http::Method>,
    //     header_mapping: Option<fn(resource: Resource, method: http::Method) -> http::HeaderMap>,
    //     client: Option<reqwest::Client>,
    // ) -> Self;

    fn query<B: Serialize>(self, resource: Resource) -> QueryBuilder<B>;

}

pub struct Client {

    pub(crate) client: reqwest::Client,

    pub(crate) api_token: fn(resource: Resource) -> String,

    pub(crate) url_generation_fn: fn(
        resource: Resource,
        method: http::Method,
        api_token: String,
        path_parameters: HashMap<String, String>,
        query_parameters: HashMap<String, String>,
    ) -> String,

    pub(crate) operation_mapping: fn(resource: Resource) -> http::Method,

    pub(crate) header_mapping: fn(resource: Resource, method: http::Method) -> http::HeaderMap,

}

impl Client {

    #[allow(dead_code)]
    pub fn new(
        api_token: fn(Resource) -> String,
        url_generation_fn: Option<
            fn(
                resource: Resource,
                method: http::Method,
                api_token: String,
                path_parameters: HashMap<String, String>,
                query_parameters: HashMap<String, String>,
            ) -> String,
        >,
        operation_mapping: Option<fn(resource: Resource) -> http::Method>,
        header_mapping: Option<fn(resource: Resource, method: http::Method) -> http::HeaderMap>,
        client: Option<reqwest::Client>,
    ) -> Self {
        Client {
            api_token,
            url_generation_fn: url_generation_fn.unwrap_or(default::url_generation_fn),
            operation_mapping: operation_mapping.unwrap_or(default::operation_mapping),
            header_mapping: header_mapping.unwrap_or(default::header_mapping),
            client: client.unwrap_or(reqwest::Client::new()),
        }
    }

}

impl Client {

    pub fn query<B: Serialize>(self, resource: Resource) -> QueryBuilder<B> {
        QueryBuilder {
            client: self, 
            resource, 
            headers: http::HeaderMap::new(),
            path_parameters: HashMap::new(),
            query_parameters: HashMap::new(),
            body: None
        }
    }

}
