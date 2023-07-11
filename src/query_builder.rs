use crate::{client::Client, query::Query, resource::Resource};
use http::{HeaderMap, HeaderValue};
use serde::Serialize;
use std::collections::HashMap;

pub trait IQueryBuilder<B: Serialize> {
    
    fn add_header(self, key: &'static str, value: &'static str) -> QueryBuilder<B>;

    fn remove_header(self, key: impl AsRef<str>) -> QueryBuilder<B>;

    fn add_query_parameter(self, key: String, value: String) -> QueryBuilder<B>;

    fn remove_query_parameter(self, key: String, value: String) -> QueryBuilder<B>;

    fn add_path_parameter(self, key: String, value: String) -> QueryBuilder<B>;

    fn remove_path_parameter(self, key: String, value: String) -> QueryBuilder<B>;

    fn body(self, body: B) -> QueryBuilder<B>;

    fn build(self) -> Query;
    
}

pub struct QueryBuilder<B: Serialize> {
    
    pub(crate) client: Client,
    
    pub(crate) resource: Resource,
    
    pub(crate) headers: HeaderMap<HeaderValue>,
    
    pub(crate) query_parameters: HashMap<String, String>,
    
    pub(crate) path_parameters: HashMap<String, String>,
    
    pub(crate) body: Option<B>,

}

impl<B: Serialize> /*IQueryBuilder<B> for*/ QueryBuilder<B> {

    pub fn add_header(mut self, key: &'static str, value: &'static str) -> QueryBuilder<B> {
        self.headers.insert(key, value.parse().unwrap());
        self
    }

    pub fn remove_header(mut self, key: impl AsRef<str>) -> QueryBuilder<B> {
        self.headers.remove(key.as_ref());
        self
    }

    pub fn add_query_parameter(mut self, key: String, value: String) -> QueryBuilder<B> {
        self.query_parameters.insert(key, value);
        self
    }

    pub fn remove_query_parameter(mut self, key: String, _value: String) -> QueryBuilder<B> {
        self.query_parameters.remove(&key);
        self
    }

    pub fn add_path_parameter(mut self, key: String, value: String) -> QueryBuilder<B> {
        self.path_parameters.insert(key, value);
        self
    }

    pub fn remove_path_parameter(mut self, key: String, _value: String) -> QueryBuilder<B> {
        self.path_parameters.remove(&key);
        self
    }

    pub fn body(mut self, body: B) -> QueryBuilder<B> {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Query {
        match (self.client.operation_mapping)(self.resource) {
            http::Method::GET => {
                let request_builder = self
                    .client
                    .client
                    .get((self.client.url_generation_fn)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                        (self.client.api_token)(self.resource),
                        self.path_parameters,
                        self.query_parameters,
                    ))
                    .headers((self.client.header_mapping)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                    ))
                    .headers(self.headers);
                Query { request_builder }
            }
            http::Method::POST => {
                let request_builder = self
                    .client
                    .client
                    .post((self.client.url_generation_fn)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                        (self.client.api_token)(self.resource),
                        self.path_parameters,
                        self.query_parameters,
                    ))
                    .headers((self.client.header_mapping)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                    ))
                    .headers(self.headers)
                    .body(serde_json::to_string(&self.body).unwrap());
                Query { request_builder }
            }
            http::Method::PUT => {
                let request_builder = self
                    .client
                    .client
                    .post((self.client.url_generation_fn)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                        (self.client.api_token)(self.resource),
                        self.path_parameters,
                        self.query_parameters,
                    ))
                    .headers((self.client.header_mapping)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                    ))
                    .headers(self.headers)
                    .body(serde_json::to_string(&self.body).unwrap());
                Query { request_builder }
            }
            http::Method::PATCH => {
                let request_builder = self
                    .client
                    .client
                    .post((self.client.url_generation_fn)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                        (self.client.api_token)(self.resource),
                        self.path_parameters,
                        self.query_parameters,
                    ))
                    .headers((self.client.header_mapping)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                    ))
                    .headers(self.headers)
                    .body(serde_json::to_string(&self.body).unwrap());
                Query { request_builder }
            }
            http::Method::DELETE => {
                let request_builder = self
                    .client
                    .client
                    .post((self.client.url_generation_fn)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                        (self.client.api_token)(self.resource),
                        self.path_parameters,
                        self.query_parameters,
                    ))
                    .headers((self.client.header_mapping)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                    ))
                    .headers(self.headers)
                    .body(serde_json::to_string(&self.body).unwrap());
                Query { request_builder }
            }
            http::Method::HEAD => {
                let request_builder = self
                    .client
                    .client
                    .post((self.client.url_generation_fn)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                        (self.client.api_token)(self.resource),
                        self.path_parameters,
                        self.query_parameters,
                    ))
                    .headers((self.client.header_mapping)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                    ))
                    .headers(self.headers)
                    .body(serde_json::to_string(&self.body).unwrap());
                Query { request_builder }
            }
            http::Method::CONNECT => {
                let request_builder = self
                    .client
                    .client
                    .post((self.client.url_generation_fn)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                        (self.client.api_token)(self.resource),
                        self.path_parameters,
                        self.query_parameters,
                    ))
                    .headers((self.client.header_mapping)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                    ))
                    .headers(self.headers)
                    .body(serde_json::to_string(&self.body).unwrap());
                Query { request_builder }
            }
            http::Method::OPTIONS => {
                let request_builder = self
                    .client
                    .client
                    .post((self.client.url_generation_fn)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                        (self.client.api_token)(self.resource),
                        self.path_parameters,
                        self.query_parameters,
                    ))
                    .headers((self.client.header_mapping)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                    ))
                    .headers(self.headers)
                    .body(serde_json::to_string(&self.body).unwrap());
                Query { request_builder }
            }
            http::Method::TRACE => {
                let request_builder = self
                    .client
                    .client
                    .post((self.client.url_generation_fn)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                        (self.client.api_token)(self.resource),
                        self.path_parameters,
                        self.query_parameters,
                    ))
                    .headers((self.client.header_mapping)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                    ))
                    .headers(self.headers)
                    .body(serde_json::to_string(&self.body).unwrap());
                Query { request_builder }
            }
            _ => {
                let request_builder = self
                    .client
                    .client
                    .post((self.client.url_generation_fn)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                        (self.client.api_token)(self.resource),
                        self.path_parameters,
                        self.query_parameters,
                    ))
                    .headers((self.client.header_mapping)(
                        self.resource,
                        (self.client.operation_mapping)(self.resource),
                    ))
                    .headers(self.headers)
                    .body(serde_json::to_string(&self.body).unwrap());
                Query { request_builder }
            }
        }
    }
}
