use reqwest::header::{HeaderMap, HeaderValue, CONNECTION, ACCEPT, USER_AGENT};
use reqwest::StatusCode;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub async fn get_request(url: &str) -> Result<StatusCode, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONNECTION, HeaderValue::from_static("close"));
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"));

    let client = reqwest::Client::new();
    let res = client.get(url)
        .headers(headers)
        .send()
        .await?;

    Ok(res.status())
}

pub fn save_to_csv(data:&String,file_name:&str){
  let binding = file_name.to_owned() +".csv";
  let path = Path::new(&binding);
  let display = path.display();
  let mut file = match File::create(&path){
    Err(why) => panic!("couldn't create {}: {}", display, why),
    Ok(file) => file,
  };
  match file.write_all(data.as_bytes()){
    Err(e) => panic!("couldn't write to {}: {}",display,e),
    Ok(_) => println!("File Save Done")
  }
}