use crate::request::TokenSourceRequest;
use crate::request::TokenSourceFetchOptions;
use crate::response::TokenSourceResponse;
use crate::response::TokenSourceResult;
use crate::error::TokenSourceError;

const SANDBOX_ENDPOINT_URL: &str = "https://cloud-api.livekit.io/api/v2/sandbox/connection-details";
const SANDBOX_ID_HEADER: &str = "X-Sandbox-ID";

pub struct TokenSourceLiteral {
    result: TokenSourceResult<TokenSourceResponse>
}

impl TokenSourceLiteral {
    pub fn new(response: TokenSourceResponse) -> TokenSourceLiteral {
        TokenSourceLiteral { result: Ok(response) }
    }
    pub fn fetch(&self) -> &TokenSourceResult<TokenSourceResponse> { &self.result }
}

pub struct TokenSourceEndpoint {
    endpoint_url: String,
    headers: Vec<(String, String)>,
    http_client: reqwest::Client,
}

impl TokenSourceEndpoint {
    pub fn new(endpoint_url: impl Into<String>, headers: Vec<(String, String)>) -> TokenSourceEndpoint {
        let http_client = reqwest::Client::new();
        
        TokenSourceEndpoint{
            endpoint_url: endpoint_url.into(), 
            headers,
            http_client
        }
    }

    pub async fn fetch(&self, options: &TokenSourceFetchOptions) -> TokenSourceResult<TokenSourceResponse> {
        let request = TokenSourceRequest::from(options);
        
        
        let mut request_builder = self.http_client
            .post(self.endpoint_url.as_str())
            .json(&request);
            
        for (name, value) in &self.headers {
            request_builder = request_builder.header(name, value);
        }

        let response = request_builder.send().await?;
        
        if !response.status().is_success() {
            return Err(TokenSourceError::Server { 
                status: response.status().as_u16(), 
                body: response.text().await.unwrap_or_default() 
            });
        }  

        let connection_details = response.json::<TokenSourceResponse>().await?;
        Ok(connection_details)
    }
}

pub struct TokenSourceSandbox {
    token_source_endpoint: TokenSourceEndpoint
}

impl TokenSourceSandbox {
    pub fn new(sandbox_id: String) -> TokenSourceSandbox { 
        let token_source_endpoint = TokenSourceEndpoint::new(
            SANDBOX_ENDPOINT_URL,
            vec![(SANDBOX_ID_HEADER.to_string(), sandbox_id)]
        );
        
        TokenSourceSandbox { 
            token_source_endpoint
        }
    }
    pub async fn fetch(&self, options: &TokenSourceFetchOptions) ->  TokenSourceResult<TokenSourceResponse> {
        self.token_source_endpoint.fetch(options).await
    }
}