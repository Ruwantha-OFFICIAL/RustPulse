mod libs;
mod core;

use core::core::calculet;
use std::env;

#[tokio::main]
async fn main(){
  let args: Vec<String> = env::args().collect();
  let mut url:String = String::new();
  let mut count:String = String::from("10");
  for (i,item) in args.iter().enumerate() {
    if item == "-url"{
      url = args[i+1].clone();
    }
    if item == "-count" {
      count = args[i+1].clone();
    }
  }
  let count_int:i32 = count.clone().parse().unwrap_or(50);
  if url.is_empty(){
    println!("Pass To agrments \"-url\".");
    return
  }
  println!("\n\n");
  calculet(&url, count_int).await;
  return;
}