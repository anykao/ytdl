#[macro_use]
extern crate quicli;
use quicli::prelude::*;
#[macro_use]
extern crate nom;
extern crate reqwest;
extern crate url;

use std::collections::HashMap;
use std::fs;
use url::Url;

// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {}

main!(|args: Cli| {
    let video_url = parse()?;
    println!("{:?}", video_url);
    println!("{:?}", args);
});

fn parse() -> Result<String> {
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

fn parse_url(qs: &str) -> Result<HashMap<String, String>> {
    let url = Url::parse(format!("https://example.com?{}", qs).as_str())?;
    let mapping: HashMap<_, _> = url.query_pairs().into_owned().collect();
    return Ok(mapping);
}

named!(
    ytconfig<&str, &str>,
    do_parse!(
        ytplayer: take!(1) >>
        (ytplayer)
    )
);

fn parse_dash(vid: &str) -> Result<String> {
    // let body = reqwest::get(format!("https://www.youtube.com/watch?v={}", vid).as_str())?.text()?;
    // let filename = format!("{}.html", vid);
    let body = fs::read_to_string("./pXwfDZLKYm8.html")?;
    println!("{}", body);
    // fs::write(filename, body.as_str())?;

    let yt = ytconfig("safsdf")?;
    println!("{:?}", yt);
    Ok(String::from(""))
}
