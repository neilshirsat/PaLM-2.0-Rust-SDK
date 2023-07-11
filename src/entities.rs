use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    pub name: String,
    pub base_model_id: Option<String>,
    pub version: String,
    pub display_name: String,
    pub description: String,
    pub input_token_limit: i32,
    pub output_token_limit: i32,
    pub supported_generation_methods: Vec<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePrompt {
    pub context: Option<String>,
    pub examples: Option<Vec<Example>>,
    pub messages: Option<Vec<Message>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    pub input: Message,
    pub output: Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub author: String,
    pub content: String,
    pub citation_metadata: Option<CitationMetadata>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    pub citation_sources: CitationSources,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSources {
    pub start_index: i32,
    pub end_index: i32,
    pub uri: String,
    pub license: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentFilter {
    pub reason: BlockedReason,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlockedReason {
    BlockedReasonUnspecified, // = "BLOCKED_REASON_UNSPECIFIED",
    Safety,                   // = "SAFETY",
    Other,                    // = "OTHER"
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountMessageTokensRequest {
    pub prompt: MessagePrompt,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountMessageTokensResponse {
    pub token_count: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedTextRequest {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedTextResponse {
    pub embedding: Embedding,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Embedding {
    pub value: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateMessageRequest {
    pub prompt: MessagePrompt,
    pub temperature: Option<f32>,
    pub candidate_count: Option<i32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateMessageResponse {
    pub candidates: Vec<Message>,
    pub messages: Vec<Message>,
    pub filter: Option<Vec<ContentFilter>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTextRequest {
    pub prompt: TextPrompt,
    pub safety_settings: Option<Vec<SafetySetting>>,
    pub stop_sequences: Option<Vec<String>>,
    pub temperature: Option<f32>,
    pub candidate_count: Option<i8>,
    pub max_output_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPrompt {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold
}


#[derive(Debug, Serialize, Deserialize)]
pub enum HarmCategory {
    #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "HARM_CATEGORY_DEROGATORY")]
    Derogatory,
    #[serde(rename = "HARM_CATEGORY_TOXICITY")]
    Toxicity,
    #[serde(rename = "HARM_CATEGORY_VIOLENCE")]
    Violence,
    #[serde(rename = "HARM_CATEGORY_SEXUAL")]
    Sexual,
    #[serde(rename = "HARM_CATEGORY_MEDICAL")]
    Medical,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS")]
    Dangerous,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HarmBlockThreshold {
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
    LowAndAbove,
    #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
    MediumAndAbove,
    #[serde(rename = "BLOCK_ONLY_HIGH")]
    OnlyHigh,
    #[serde(rename = "BLOCK_NONE")]
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "NEGLIGIBLE")]
    Negligible,
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateTextResponse {
    pub candidates: Vec<TextCompletion>,
    pub filter: Option<Vec<ContentFilter>>,
    pub safety_feedback: Option<Vec<SafetyFeedback>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextCompletion {
    pub output: String,
    pub safety_ratings: Vec<SafetyRating>,
    pub citation_metadata: Option<CitationMetadata>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyFeedback {
    pub rating: SafetyRating,
    pub setting: SafetySetting
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInfoResponse {
    pub models: Vec<ModelInfo>,
    pub next_page_token: Option<String>
}