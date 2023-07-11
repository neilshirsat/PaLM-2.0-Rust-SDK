use crate::resource::Resource;
use std::collections::HashMap;

#[allow(dead_code)]
pub(super) fn url_generation_fn(
    resource: Resource,
    _method: http::Method,
    api_token: String,
    path_parameters: HashMap<String, String>,
    query_parameters: HashMap<String, String>,
) -> String {
    const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta2/models";

    match resource {
        Resource::CountMessageTokens => {
            let model_info = path_parameters.get("model").unwrap();
            format!(
                "{}/{}:countMessageTokens?key={}",
                BASE_URL, model_info, api_token
            )
        }
        Resource::EmbedText => {
            let model_info = path_parameters.get("model").unwrap();
            format!(
                "{}/{}:embedText?key={}",
                BASE_URL, model_info, api_token
            )
        },
        Resource::GenerateMessage => {
            let model_info = path_parameters.get("model").unwrap();
            format!(
                "{}/{}:generateMessage?key={}",
                BASE_URL, model_info, api_token
            )
        },
        Resource::GenerateText => {
            let model_info = path_parameters.get("model").unwrap();
            format!(
                "{}/{}:generateText?key={}",
                BASE_URL, model_info, api_token
            )
        },
        Resource::GetModelInfo => {
            let model_info = path_parameters.get("model").unwrap();
            format!(
                "{}/{}?key={}",
                BASE_URL, model_info, api_token
            )
        },
        Resource::ListInfo => {
            match query_parameters.len() {
                1 => {
                    let query_param = query_parameters.keys().next().unwrap();
                    let query_value = query_parameters.get(query_param).unwrap();
                    format!(
                        "{}?key={}&{}={}",
                        BASE_URL, api_token, query_param, query_value
                    )
                },
                2 => {
                    let query_param_1 = query_parameters.keys().next().unwrap();
                    let query_value_1 = query_parameters.get(query_param_1).unwrap();
                    let query_param_2 = query_parameters.keys().next().unwrap();
                    let query_value_2 = query_parameters.get(query_param_2).unwrap();
                    format!(
                        "{}?key={}&{}={}&{}={}",
                        BASE_URL, api_token, query_param_1, query_value_1, query_param_2, query_value_2
                    )
                },
                _ => format!(
                        "{}?key={}",
                        BASE_URL, api_token
                    )
            }
        },
    }
}

#[allow(dead_code)]
pub(super) fn operation_mapping(resource: Resource) -> http::Method {
    match resource {
        Resource::CountMessageTokens => http::Method::POST,
        Resource::EmbedText => http::Method::POST,
        Resource::GenerateMessage => http::Method::POST,
        Resource::GenerateText => http::Method::POST,
        Resource::GetModelInfo => http::Method::GET,
        Resource::ListInfo => http::Method::GET,
    }
}

#[allow(dead_code)]
pub(super) fn header_mapping(_resource: Resource, _method: http::Method) -> http::HeaderMap {
    let mut header_map = http::HeaderMap::new();
    header_map.insert(http::header::ACCEPT, "application/json".parse().unwrap());
    header_map
}
