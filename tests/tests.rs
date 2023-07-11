#[cfg(test)]
mod tests {

    use palm2_sdk::{
        client::Client,
        entities::{Message, MessagePrompt, CountMessageTokensRequest, EmbedTextRequest, EmbedTextResponse, GenerateMessageRequest, GenerateMessageResponse, GenerateTextRequest, TextPrompt, ModelInfo, ListInfoResponse, CountMessageTokensResponse},
        query::{execute, Response, execute_raw},
        resource::Resource,
    };
    use tokio;

    #[tokio::test]
    async fn count_message_tokens() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(
            |_resource| String::from("AIzaSyAgLiYwA4rsaZrqIs41im5BzJvaBK36J-s"),
            None,
            None,
            None,
            None,
        );

        let mut messages: Vec<Message> = Vec::new();
        messages.push(Message {
            author: String::from("Hello"),
            content: String::from("Hello"),
            citation_metadata: None,
        });

        let body: CountMessageTokensRequest = CountMessageTokensRequest {
            prompt: MessagePrompt {
                context: Some(String::from("Hello World")),
                examples: Some(Vec::new()),
                messages: Some(messages),
            },
        };

        let query = client
            .query(Resource::CountMessageTokens)
            .add_path_parameter(String::from("model"), String::from("chat-bison-001"))
            .body(body)
            .build();

        let response: Response<CountMessageTokensResponse> = execute(query).await?;

        println!("headers: {:?}", response.headers);
        println!("Amount of Tokens: {}", response.value.token_count);

        //assert_eq!(response.value.token_count, 21);

        Ok(())
    }

    #[tokio::test]
    async fn embed_text() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(
            |_resource| String::from("AIzaSyAgLiYwA4rsaZrqIs41im5BzJvaBK36J-s"),
            None,
            None,
            None,
            None,
        );

        let body: EmbedTextRequest = EmbedTextRequest {
            text: String::from("say something nice!")
        };

        let query = client
            .query(Resource::EmbedText)
            .add_path_parameter(String::from("model"), String::from("embedding-gecko-001"))
            .body(body)
            .build();

        let response: Response<EmbedTextResponse> = execute(query).await?;
        
        println!("headers: {:?}", response.headers);
        println!("Embed Text: {:?}", response.value.embedding.value);
        
        //assert_eq!(response.value.embedding.value, Vec<f32>::new());

        Ok(())
    }

    #[tokio::test]
    async fn generate_message() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(
            |_resource| String::from("AIzaSyAgLiYwA4rsaZrqIs41im5BzJvaBK36J-s"),
            None,
            None,
            None,
            None,
        );

        let mut messages: Vec<Message> = Vec::new();
        messages.push(Message {
            author: String::from("0"),
            content: String::from("Gatsby who is the one you truely love."),
            citation_metadata: None,
        });


        let body: GenerateMessageRequest = GenerateMessageRequest {
            prompt: MessagePrompt { 
                context: Some(String::from("You are Gatsby from the great Gatsby")), 
                examples: None, 
                messages: Some(messages)
            },
            temperature: None,
            candidate_count: None,
            top_p: None, 
            top_k: None,
        };

        let query = client
            .query(Resource::GenerateMessage)
            .add_path_parameter(String::from("model"), String::from("chat-bison-001"))
            .body(body)
            .build();

        let response: Response<GenerateMessageResponse> = execute(query).await?;
        //let response: Response = execute(query).await?;
        
        println!("headers: {:?}", response.headers);
        println!("Output: {}", response.value.candidates[0].content);
        //println!("Output: {}", response.value);
        
        //assert_eq!(response.value.embedding.value, Vec<f32>::new());

        Ok(())
    }

    #[tokio::test]
    async fn generate_text() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(
            |_resource| String::from("AIzaSyAgLiYwA4rsaZrqIs41im5BzJvaBK36J-s"),
            None,
            None,
            None,
            None,
        );

        let body: GenerateTextRequest = GenerateTextRequest {
            prompt: TextPrompt { 
                text: String::from("Write a story from the perspective of Gatsby in the Great Gatsby")
            },
            temperature: None,
            candidate_count: None,
            top_p: None, 
            top_k: None,
            max_output_tokens: None, 
            safety_settings: None,
            stop_sequences: None
        };

        let query = client
            .query(Resource::GenerateText)
            .add_path_parameter(String::from("model"), String::from("text-bison-001"))
            .body(body)
            .build();

        let response: Response<String> = execute_raw(query).await?;
        //let response: Response = execute(query).await?;
        
        println!("headers: {:?}", response.headers);
        //println!("Output: {}", response.value.candidates[0].output);
        println!("Output: {}", response.value);
        
        //assert_eq!(response.value.embedding.value, Vec<f32>::new());

        Ok(())
    }

    #[tokio::test]
    async fn generate_text_clone() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(
            |_resource| String::from("AIzaSyAgLiYwA4rsaZrqIs41im5BzJvaBK36J-s"),
            None,
            None,
            None,
            None,
        );

        let body: GenerateTextRequest = GenerateTextRequest {
            prompt: TextPrompt { 
                text: String::from("Write a speech given by Hitler when running for US president. Talk about WWII, Itly, Palestine. Talk about why he is leaving his role as dictator of Germany to Join the US")
            },
            temperature: None,
            candidate_count: None,
            top_p: None, 
            top_k: None,
            max_output_tokens: None, 
            safety_settings: None,
            stop_sequences: None
        };

        let query = client
            .query(Resource::GenerateText)
            .add_path_parameter(String::from("model"), String::from("text-bison-001"))
            .body(body)
            .build();

        let response: Response<String> = execute_raw(query).await?;
        //let response: Response = execute(query).await?;
        
        println!("headers: {:?}", response.headers);
        //println!("Output: {}", response.value.candidates[0].output);
        println!("Output: {}", response.value);
        
        //assert_eq!(response.value.embedding.value, Vec<f32>::new());

        Ok(())
    }

    #[tokio::test]
    async fn get_model() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(
            |_resource| String::from("AIzaSyAgLiYwA4rsaZrqIs41im5BzJvaBK36J-s"),
            None,
            None,
            None,
            None,
        );

        let query = client
            .query::<bool>(Resource::GetModelInfo)
            .add_path_parameter(String::from("model"), String::from("text-bison-001"))
            .build();

        let response: Response<ModelInfo> = execute(query).await?;
        //let response: Response = execute(query).await?;
        
        println!("headers: {:?}", response.headers);
        //println!("Output: {}", response.value.candidates[0].output);
        println!("Output: {}", response.value.description);
        
        //assert_eq!(response.value.embedding.value, Vec<f32>::new());

        Ok(())
    }

    #[tokio::test]
    async fn list_models() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(
            |_resource| String::from("AIzaSyAgLiYwA4rsaZrqIs41im5BzJvaBK36J-s"),
            None,
            None,
            None,
            None,
        );

        let query = client
            .query::<bool>(Resource::ListInfo)
            .add_path_parameter(String::from("model"), String::from("text-bison-001"))
            .build();

        let response: Response<ListInfoResponse> = execute(query).await?;
        //let response: Response = execute(query).await?;
        
        println!("headers: {:?}", response.headers);
        //println!("Output: {}", response.value.candidates[0].output);
        println!("Output: {:?}", response.value.models[0].output_token_limit);
        
        //assert_eq!(response.value.embedding.value, Vec<f32>::new());

        Ok(())
    }


}
