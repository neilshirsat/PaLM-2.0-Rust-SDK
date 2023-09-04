# Rust PaLM 2.0 SDK

The Rust PaLM 2.0 SDK is a software development kit that allows
developers to easily access the PALM 2.0 API without having to 
manage induvidual API calls. 

This library is a wrapper of the ```reqwest``` library. 

## Installation

### Command Line

In your cargo project run the command:

```bash
cargo add palm2_sdk
```

### Cargo.toml

In your cargo.toml add the following line:
```toml
palm2_sdk = "0.1.0" 
```

Then run the following command:
```bash
cargo build
```

## Usage

First inialize the client:

```rust
let client = Client::new(
    |_resource| String::from("$API_KEY"),
    None,
    None,
    None,
    None,
);
```

This library handels api_keys in a different way. API Keys are provided in the form a callback function or a closure. This allows keys to be provided on a per-resource level. 

This library allows organizations to build and add custom business logic to the application in the form of call back functions that are specified to the client. Otherwise, a None should be specified. Specifically the library allows the client to inject a function that customizes the Http Method based on the resource fetched, a function that specificies a global header map at a resource level, a function that generates the URL to query from based on the query and path parameters, api token, and method. Additionally, an optional ```reqwest::Client``` can be provided to the application to customize the client used to fetch the resource. 

In order to query data use the function ```query``` of ```Client``` to generate
a ```Query```. 

First generate the Input Data for the Query. All request, response, and core entities are located in ```use palm2_sdk::entities```. 

```rust
let input: GenerateTextRequest = GenerateTextRequest {
    prompt: TextPrompt { 
        text: String::from("Write a story from the perspective of Mickey Mouse")
    },
    temperature: None,
    candidate_count: None,
    top_p: None, 
    top_k: None,
    max_output_tokens: None, 
    safety_settings: None,
    stop_sequences: None
};
```

Then create the ```Query```
```rust
let query = client
    .query(Resource::GenerateText)
    .add_path_parameter(String::from("model"), String::from("text-bison-001"))
    .body(input)
    .build();
```

In order to fetch the resource and obtain the response use the ```execute``` and ```execute_raw``` functions (which are ```async```). 

```rust
let response: Response<GenerateTextResponse> = execute(query).await?;

println!("Generated Ouput: {}", response.value.candidates[0].output);
```


```rust
let response: Response<String> = execute_raw(query).await?;

println!("Raw Output: {}", response.value);
```

```execute``` deseializes the response data into a struct allowing its values to be used by the program.

```execute_raw``` keeps the response data in a string representation. Use this function over the ```execute``` function when the library is being it used in a reverse proxy since proxies do not need to deserialize and reserialize the data. Essentially, ```execute_raw``` reduces the overheat and improves performance of the library in a proxy enviornment.  

## Credits

(c) Neil Shirsat
