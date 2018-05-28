#[macro_use]
extern crate nom;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;
#[macro_use]
extern crate log;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate rayon;

pub mod downloader;

mod decipher;
mod douyin;
mod youtube;

use std::collections::HashMap;
use std::error::Error;
use url::Url;

#[derive(Debug)]
pub enum Provider {
    Youtube(String),
    Douyin(String),
}

#[derive(Debug)]
pub enum VideoUrl {
    Failed,
    Direct {
        url: String,
        filename: String,
    }, // (url, filename)
    Dash {
        video_url: String,
        audio_url: String,
        filename: String,
    }, // (video_url, audio_url, filename)
}

pub fn list(uid: &str) -> Vec<VideoUrl> {
    douyin::list(uid)
}

pub fn parse(p: &Provider) -> VideoUrl {
    match p {
        Provider::Youtube(id) => {
            return youtube::parse(id);
        }
        Provider::Douyin(url) => {
            return douyin::parse(url);
        }
    }
}

fn parse_url(qs: &str) -> Result<HashMap<String, String>, Box<Error>> {
    let url = Url::parse(format!("https://example.com?{}", qs).as_str())?;
    let mapping: HashMap<_, _> = url.query_pairs().into_owned().collect();
    return Ok(mapping);
}

fn sanitize(original: &str, replacement: Option<char>) -> String {
    match replacement {
        Some(c) => original
            .chars()
            .map(|x| match x {
                '/' | '?' | '<' | '>' | '\\' | ':' | '*' | '|' | '"' => c,
                '\x00'...'\x1f' => c,
                _ => x,
            })
            .collect(),
        None => original
            .chars()
            .filter(|x| match x {
                '/' | '?' | '<' | '>' | '\\' | ':' | '*' | '|' | '"' => false,
                '\x00'...'\x1f' => false,
                _ => true,
            })
            .collect(),
    }
}
