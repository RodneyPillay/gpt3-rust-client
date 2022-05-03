#![allow(non_snake_case)]

#[macro_use]
extern crate clap;

use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::io::{stdin, stdout, Write};
use clap::App;

#[derive(Deserialize, Debug, Clone)]
struct OpenAIChoices {
    text: String,
}

#[derive(Deserialize, Debug, Clone)]
struct OpenAIResponse {
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
    
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input_temperature = matches.value_of("temperature").unwrap_or("0.7");
    let input_max_tokens = matches.value_of("temperature").unwrap_or("100");
    
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";

    let openai_token: String = env::var("OPENAI_TOKEN")?;
    let auth_header_val = format!("Bearer {}", openai_token);
    
    loop {
        print!("\n{}\n> ", "Ask GPT3 Anything");
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
            prompt: format!("{}", user_text),
            temperature: input_temperature.parse().unwrap(),
            max_tokens: input_max_tokens.parse().unwrap(), 
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
