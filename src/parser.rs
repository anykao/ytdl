use super::Provider;
use decipher::decipher;
use nom::types::CompleteStr;
use reqwest;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use url::Url;

named!(
    ytconfig<CompleteStr, CompleteStr>,
    do_parse!(
        take_until_and_consume!("ytplayer.config") >>
        ws!(tag!("=")) >>
        obj: take_until!(";ytplayer.load") >>
        (obj)
    )
);

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

#[derive(Debug)]
pub enum VideoUrl {
    Direct(String),
    Dash(String, String),
}

pub fn parse(p: &Provider) -> Result<VideoUrl, Box<Error>> {
    // let body = reqwest::get(&format!(
    //     "http://youtube.com/get_video_info?video_id={}",
    //     vid
    // ))?.text()?;
    // let mapping = parse_url(body.as_str())?;
    // if let Some(v) = mapping.get("url_encoded_fmt_stream_map") {
    //     for d in v.split(",") {
    //         let m2 = parse_url(d)?;
    //         if let Some(url) = m2.get("url") {
    //             return Ok(VideoUrl::Direct(url.to_string()));
    //         } else {
    //             let pair = parse_dash(vid);
    //             return Ok(VideoUrl::Dash(pair.0, pair.1));
    //         }
    //     }
    // } else {
    //     unreachable!("dash")
    // }

    match p {
        Provider::Youtube(id) => {
            let pair = parse_dash(id);
            return Ok(VideoUrl::Dash(pair.0, pair.1));
        }
        Provider::Douyin(id) => {
            return Ok(parse_douyin(id));
        }
    }
}

fn parse_douyin(url: &str) -> VideoUrl {
    let body = reqwest::get(url).unwrap().text().unwrap();
    println!("{}", body);
    VideoUrl::Direct("https://aweme.snssdk.com/aweme/v1/playwm/?video_id=44e3f61f5b2542e1b600be67bd4ee584&line=0".to_string())
}

fn parse_url(qs: &str) -> Result<HashMap<String, String>, Box<Error>> {
    let url = Url::parse(format!("https://example.com?{}", qs).as_str())?;
    let mapping: HashMap<_, _> = url.query_pairs().into_owned().collect();
    return Ok(mapping);
}

#[test]
fn test_parse_douyin() {
    parse_douyin("https://www.tiktokv.com/i18n/share/video/6560042923969219841");
}
