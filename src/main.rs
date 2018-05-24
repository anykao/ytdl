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

use nom::types::CompleteStr;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use url::Url;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "ytdl")]
struct Opt {}

fn main() -> Result<(), Box<Error>> {
    // let video_url = parse()?;
    // println!("{:?}", video_url);
    decipher("", "").unwrap();
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
                return parse_dash("pXwfDZLKYm8");
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

#[derive(Debug, Deserialize)]
struct Assets {
    js: String,
}

#[derive(Debug, Deserialize)]
struct Args {
    adaptive_fmts: String,
}

#[derive(Debug, Deserialize)]
struct PlayerConfig {
    assets: Assets,
    args: Args,
}

fn parse_dash(vid: &str) -> Result<String, Box<Error>> {
    named!(
        ytconfig<CompleteStr, CompleteStr>,
        do_parse!(
            take_until_and_consume!("ytplayer.config") >>
            ws!(tag!("=")) >>
            obj: take_until!(";ytplayer.load") >>
            (obj)
        )
    );
    // let body = reqwest::get(format!("https://www.youtube.com/watch?v={}", vid).as_str())?.text()?;
    // let filename = format!("{}.html", vid);
    // let body = fs::read_to_string("./pXwfDZLKYm8.html")?;
    // println!("{}", body);
    // fs::write(filename, body.as_str())?;
    let body = fs::read_to_string("./pXwfDZLKYm8.html")?;
    let (_, json) = ytconfig(CompleteStr(body.as_str())).unwrap();
    let config: PlayerConfig = serde_json::from_str(&json).unwrap();
    println!("{:#?}", config);

    // let mut resp = reqwest::get(format!("https://www.youtube.com/{}", config.assets.js).as_str())?;
    // let mut file = File::create("script.js").unwrap();
    // resp.copy_to(&mut file)?;

    Ok(String::new())
}

fn decipher(_js: &str, s: &str) -> Result<String, Box<Error>> {
    named!(
        parser<CompleteStr, CompleteStr>,
        do_parse!(
            take_until_and_consume!(r#""signature","#) >>
            f: take_until!("(") >>
            delimited!( tag!("("), take_until!(")"), tag!(")"))>>
            (f)
        )
    );
    named!(
        fdef<CompleteStr, CompleteStr>,
        do_parse!(
            take_until_and_consume!(r#"pL=function"#) >>
            arg: delimited!( tag!("("), take_until!(")"), tag!(")"))>>
            body: delimited!( tag!("{"), take_until!("}"), tag!("}"))>>
            (body)
        )
    );
    let script = fs::read_to_string("./script.js")?;
    let (_, f) = parser(CompleteStr(script.as_str())).unwrap();
    let (_, def) = fdef(CompleteStr(script.as_str())).unwrap();
    println!("{}", f);
    println!("{}", def);
    Ok(String::new())
}

fn get_dash_url() {}
