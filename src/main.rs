#![allow(dead_code)]
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate nom;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate url;
#[macro_use]
extern crate log;
extern crate fern;

mod decipher;
mod logger;
mod parser;

use parser::parse_dash;
use std::collections::HashMap;
use std::error::Error;
use url::Url;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "ytdl")]
struct Opt {}

fn main() -> Result<(), Box<Error>> {
    logger::init();
    // let video_url = parse()?;
    // println!("{:?}", video_url);
    decipher::decipher("", "").unwrap();
    Ok(())
}

fn parse() -> Result<String, Box<Error>> {
    let body = reqwest::get("http://youtube.com/get_video_info?video_id=pXwfDZLKYm8")?.text()?;
    let mapping = parse_url(body.as_str())?;
    if let Some(v) = mapping.get("url_encoded_fmt_stream_map") {
        for d in v.split(",") {
            let m2 = parse_url(d)?;
            if let Some(url) = m2.get("url") {
                return Ok(url.to_string());
            } else {
                return Ok(parse_dash("pXwfDZLKYm8"));
            }
        }
    } else {
        unimplemented!("dash")
    }
    Ok(String::new())
}

fn parse_url(qs: &str) -> Result<HashMap<String, String>, Box<Error>> {
    let url = Url::parse(format!("https://example.com?{}", qs).as_str())?;
    let mapping: HashMap<_, _> = url.query_pairs().into_owned().collect();
    return Ok(mapping);
}
