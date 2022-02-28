use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::io::{stdin, stdout, Write};


#[derive(Deserialize, Debug, Clone)]
struct OpenAIChoices {
    text: String,
    index: u8,
    log_probs: Option<u8>,
    finish_reason: String,
}

#[derive(Deserialize, Debug, Clone)]
struct OpenAIResponse {
    id:Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OpenAIChoices>,
}

#[derive(Serialize, Debug)]
struct OpenAIRequest {
    prompt: String,
    temperature: f32,
    max_tokens: u32,
    top_p: u32,
    frequency_penalty: u32,
    presence_penalty: u32,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";

    let preamble = "Answer the following question accurately, 
                    but find a funny way to mention 
                    the Rust programming language in your response";

    let openai_token: String = env::var("OPENAI_TOKEN")?;
    let auth_header_val = format!("Bearer {}", openai_token);

    
    loop {
        print!("\n{}\n> ", "Ask GPT3 a Question");
        stdout().flush()?;

        
        let mut user_text = String::new(); 
        
       
        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read line");
        if user_text.eq("exit\r\n") { // sort out exit.
           break;
        }

        println!("...Thinking");
        let oai_request = OpenAIRequest {
            prompt: format!("{} {}", preamble, user_text),
            temperature: 0.7,
            max_tokens: 100, 
            top_p: 1,
            frequency_penalty: 0,
            presence_penalty: 0,
        };
                
        let body =  Body::from(serde_json::to_vec(&oai_request)?);
        let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)?;

        let res = client.request(req).await?;
        let body = hyper::body::aggregate(res).await?;
        let json: OpenAIResponse = serde_json::from_reader(body.reader())?;
      
        println!("");
        println!("{}", json.choices[0].text);
    }
    Ok(())
}
