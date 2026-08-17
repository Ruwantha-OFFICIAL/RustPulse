use std::sync::Mutex;
use std::sync::Arc;
use std::time::Instant;
use tokio::task::JoinHandle;
use std::thread;
use regex::Regex;
//local file system import
use crate::libs::funtion::get_request;
use crate::libs::funtion::save_to_csv;


#[derive(Debug)]
struct X01 {
  id: i32,
  delay: f32,
}

pub async fn calculet(url: &str,count:i32) {
  let is_ready = host_ready(url).await;
  println!("Host is chekd : {}",if is_ready{"Done"}else{"Not"});
  if !is_ready {
    return
  }
  // simulate traffic - now concurrent using tokio::spawn
  let err_count = Arc::new(Mutex::new(0));
  let mut handles: Vec<JoinHandle<Option<X01>>> = Vec::new();
  println!("Wait simulate traffic..");
  for i in 1..=count {
    let url = url.to_string();
    let err_count = Arc::clone(&err_count);

    let handle = tokio::spawn(async move {
      let start = Instant::now();
      match get_request(&url).await {
        Ok(_res) => {
          let delay = start.elapsed().as_secs_f32();
          Some(X01 {
            id: i, delay
          })
        }
        Err(e) => {
          let mut ec = err_count.lock().unwrap();
          *ec += 1;
          println!("Erro :{}", e);
          None
        }
      }
    });

    handles.push(handle);
  }

  // collect results
  let mut data: Vec<X01> = Vec::new();
  for handle in handles {
    if let Ok(Some(x)) = handle.await {
      data.push(x);
    }
  }

  let final_err_count = *err_count.lock().unwrap();
  let (min, max) = anylse(&data);
  let m = mean(&data);
  let std = std_dev(&data, &m);
  let performance = performance_score(1.0, &m);
  let success_rate = (data.len() as f32 / count as f32) * 100.0;

  println!("\n╔══════════════════════════════════════╗");
  println!("║          LOAD TEST REPORT            ║");
  println!("╚══════════════════════════════════════╝");
  println!(" Target URL       : {}", url);
  println!(" Total Requests   : {}", count);
  println!(" Success          : {}", data.len());
  println!(" Failed           : {}", final_err_count);
  println!(" Success Rate     : {:.2}%", success_rate);
  println!("----------------------------------------");
  println!(" Min Response     : {:.3}s", min);
  println!(" Max Response     : {:.3}s", max);
  println!(" Mean Response    : {:.3}s", m);
  println!(" Std Deviation    : {:.3}s", std);
  println!("----------------------------------------");
  println!(" Performance Score: {:.2} / 100 {}", performance, score_badge(performance));
  println!("========================================\n");

  let re = Regex::new(r"^https://(?:www\.)?([^./]+)").unwrap();
  let name = match re.captures(url) {
    Some(caps) => caps[1].to_string(),
    None =>{
      println!("{url} must be rejectd");
      "unknown".to_string()
    },
  };
  
  let mut csv_data = String::from("id,response_time\n");
  for item in &data{
    csv_data += &format!("{},{}\n",item.id, item.delay)
  }
  save_to_csv(&csv_data,&name)//void
}

async fn host_ready(url:&str)->bool{
  let results:bool = match get_request(url).await {
    Ok(_res) => true,
    Err(_e) => false
  };
  return results
}

fn anylse(arry: &Vec<X01>)->(f32,f32){
  //Max,Min time delay
  let mut max_delay = 0.0;
  let mut min_delay = 100.0;
  for item in arry {
    if max_delay < item.delay {
      max_delay = item.delay;
    }
    if min_delay > item.delay {
      min_delay = item.delay;
    }
  }
  
  return (min_delay,max_delay);
}

//Mean (μ) = Σx / n
//Std Dev (σ) = √(Σ(x - μ)² / n)
fn mean(arry: &Vec<X01>)->f32{
  //Mean (μ) = Σx / n 
  let mut x:f32 = 0.0;
  let n = arry.len();
  for item in arry{
    x += item.delay;
  }
  let mean = x / n as f32;
  
  return mean;
}

fn std_dev(arry: &Vec<X01>,mean:&f32)->f32{
  //Std Dev (σ) = √(Σ(x - μ)² / n)
  let mut sq = 0.0;
  let n = arry.len();
  
  for item in arry {
    let dif = item.delay - mean;
    sq += dif * dif;
  }
  
  return (sq / n as f32).sqrt();
}

fn performance_score(target: f32,mean:&f32) -> f32 {
    
    let mut score = (target / mean) * 100.0;
    
    if score > 100.0 {
        score = 100.0;
    }
    
    score
}

fn score_badge(score: f32) -> &'static str {
    match score {
        s if s >= 90.0 => "🟢 Excellent",
        s if s >= 75.0 => "🟡 Good",
        s if s >= 50.0 => "🟠 Average",
        _              => "🔴 Poor",
    }
}