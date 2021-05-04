use std::env;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Conf {
    webhook_api_key: String,
    execute_event_name: String,
}

fn main() {
  let conf:Conf = read_conf(is_dev());
  get(make_url(&conf.webhook_api_key, &conf.execute_event_name));
}

fn get(call_url:String) {
  let body = reqwest::blocking::get(call_url).unwrap()
      .text().unwrap();
  println!("body = {:?}", body);
}

fn is_dev()-> bool{
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    return args[1] == "isDev"
  }else{
    return false
  }
}

fn read_conf(is_dev: bool)-> Conf{
  let filename =  if is_dev {"conf_dev.json"} else {"conf.json"};
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("something went wrong reading the file");

  return serde_json::from_str(&contents).unwrap();
}

fn make_url(api_key: &str,service_name: &str) -> String {
  return format!("https://maker.ifttt.com/trigger/{}/with/key/{}", service_name, api_key);
}