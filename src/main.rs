#[macro_use]
extern crate clap;
use anyhow::Result;
use cronjob::CronJob;
use serde::{Deserialize, Serialize};
use core::panic;
use std::fs::{self};
use rand::seq::SliceRandom;
use futures::executor::block_on;
use tokio_compat_02::FutureExt;
use clap::{App, Arg, SubCommand};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
  meme: Vec<String>,
}

fn main () {
  let app = App::new(crate_name!()).version(crate_version!()).author(crate_authors!()).about(crate_description!())
    .subcommand(SubCommand::with_name("show").about("show a meme").arg(Arg::with_name("list").help("show all memes").short("l").long("list")).arg(Arg::with_name("cron").help("show a meme at cron setting").long("cron")));
  let matches = app.get_matches();
  if let Some(ref matches) = matches.subcommand_matches("show") {
    if matches.is_present("list") {
      block_on(display_meme(true)).unwrap();
      return;
    }
    if matches.is_present("cron") {
      let mut cron = CronJob::new("meme", on_cron);
      cron.seconds("0, 15, 30, 45");
      cron.start_job();
      return;
    }
    block_on(display_meme(false)).unwrap();
  }
}

fn on_cron(_name: &str) {
  block_on(display_meme(false)).unwrap();
}

async fn display_meme (all: bool) -> Result<()> {
  let response = reqwest::get("https://raw.githubusercontent.com/subaru-git/memeinder/main/src/data/data.toml").compat().await.unwrap().text().await.unwrap();
  // let client = reqwest::Client::new();
  // let response = client.get("https://raw.githubusercontent.com/subaru-git/memeinder/main/src/data/data.toml").send().compat().await?;
  // println!("{:?}", response);
  // let toml: String = fs::read_to_string("src/data/data.toml").unwrap();
  let data: Result<Data, toml::de::Error> = toml::from_str(&response);
  match data {
    Ok(p) => {if !all {println!("{}", p.meme.choose(&mut rand::thread_rng()).unwrap())} else {println!("{:?}", p.meme)}},
    Err(e) => panic!("Faild to parse TOML: {}", e),
  }
  Ok(())
}