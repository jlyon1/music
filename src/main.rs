
extern crate reqwest;

use std::io::{self, BufRead};
use std::env;
use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;

fn next_song(key: &mut str) -> bool{
    let client = reqwest::Client::new();

    let start = "Bearer ";
    let mut res = client.post("https://api.spotify.com/v1/me/player/next")
        .header("Authorization",format!("{} {}",start,key)).header("content-length","0").send().expect("asdf");

    let mut buf = String::new();
    
    res.read_to_string(&mut buf).expect("test");

    if res.status() == 204 {
        return true;
    }
    println!("ERROR:{}",buf);

    return false;
}

fn prev_song(key: &mut str) -> bool{
    let client = reqwest::Client::new();

    let start = "Bearer ";
    let mut res = client.post("https://api.spotify.com/v1/me/player/previous")
        .header("Authorization",format!("{} {}",start,key)).header("content-length","0").send().expect("asdf");

    let mut buf = String::new();
    
    res.read_to_string(&mut buf).expect("test");

    if res.status() == 204 {
        return true;
    }
    println!("ERROR:{}",buf);

    return false;
}

fn main(){
    let mut file = File::open("conf.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let v: Value = serde_json::from_str(&data).unwrap();

    let mut key = String::new();

    key = format!("{}",v["apiKey"].as_str().unwrap());
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3{
        
        println!("Invalid args");
        return;
    }
    let mut val: i32 = 1;

    if args.len() == 3 {
        val = args[2].parse::<i32>().unwrap();

    }

    if args[1] == "next"{
        for x in 0..val{
            next_song(&mut key);
        }

    }else if args[1] == "prev"{
        for x in 0..val{
            prev_song(&mut key);

        }
    }

}
