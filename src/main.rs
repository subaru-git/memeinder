use cronjob::CronJob;
use serde::{Deserialize, Serialize};
use core::panic;
use std::fs::{self};
use rand::seq::SliceRandom;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
  meme: Vec<String>,
}

fn main () {
  let mut cron = CronJob::new("meme", on_cron);
  cron.seconds("0, 15, 30, 45");
  cron.start_job();
}

fn on_cron(_name: &str) {
  let toml: String = fs::read_to_string("src/data/data.toml").unwrap();
  let data: Result<Data, toml::de::Error> = toml::from_str(&toml);
  match data {
    Ok(p) => println!("{}", p.meme.choose(&mut rand::thread_rng()).unwrap()),
    Err(e) => panic!("Faild to parse TOML: {}", e),
  }
}