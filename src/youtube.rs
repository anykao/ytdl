use super::decipher::*;
use super::parse_url;
use super::sanitize;
use super::VideoUrl;

use nom::types::CompleteStr;
use reqwest;
use serde_json::{self, Value};
use std::io::prelude::*;
use std::process::{Command, Stdio};

named!(
    ytconfig<CompleteStr, CompleteStr>,
    do_parse!(
        take_until_and_consume!("ytplayer.config") >>
        ws!(tag!("=")) >>
        obj: take_until!(";ytplayer.load") >>
        (obj)
    )
);

pub fn parse(vid: &str) -> VideoUrl {
    let body = reqwest::get(&format!("https://www.youtube.com/watch?v={}", vid))
        .unwrap()
        .text()
        .unwrap();
    let (_, json) = ytconfig(CompleteStr(body.as_str())).unwrap();
    let v: Value = serde_json::from_str(&json).unwrap();
    let title = v.pointer("/args/title").unwrap().as_str().unwrap();
    let adaptive_fmts = v.pointer("/args/adaptive_fmts").unwrap().as_str().unwrap();
    let js = v.pointer("/assets/js").unwrap().as_str().unwrap();
    let good_title = sanitize(title, None);
    // debug!("{:#?}", config);
    let script = reqwest::get(format!("https://www.youtube.com/{}", js).as_str())
        .unwrap()
        .text()
        .unwrap();
    // let mut file = File::create("script.js").unwrap();
    // resp.copy_to(&mut file)?;
    let mut video_url = String::new();
    let mut audio_url = String::new();
    let (node_script, f) = decipher(&script).unwrap();
    let afmts = adaptive_fmts.split(",");
    for fmt in afmts {
        let afmt = parse_url(fmt).unwrap();
        debug!("{:#?}", afmt);
        if let Some(type_) = afmt.get("type") {
            if type_.starts_with("video/") && video_url.is_empty() {
                debug!("{}", type_);
                if let Some(ref s) = afmt.get("s") {
                    let sig = generate_sig(&node_script, &f, s);
                    video_url = format!("{}&signature={}", &afmt["url"], sig);
                } else {
                    video_url = afmt["url"].clone();
                }
            } else if type_.starts_with("audio/") && audio_url.is_empty() {
                debug!("{}", type_);
                if let Some(ref s) = afmt.get("s") {
                    let sig = generate_sig(&node_script, &f, s);
                    audio_url = format!("{}&signature={}", &afmt["url"], sig);
                } else {
                    audio_url = afmt["url"].clone();
                }
            } else {
                if !audio_url.is_empty() && !video_url.is_empty() {
                    return VideoUrl::Dash {
                        video_url,
                        audio_url,
                        filename: format!("{}_{}", good_title, vid),
                    };
                }
            }
        }
    }
    VideoUrl::Failed
}

fn generate_sig(temp_js: &str, f: &str, sig: &str) -> String {
    let mut bytes = String::new();
    bytes.push_str(temp_js);
    bytes.push_str(&format!(r#"console.log({}("{}"))"#, f, sig));
    let process = Command::new("node")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    process.stdin.unwrap().write_all(bytes.as_bytes()).unwrap();
    let mut s = String::new();
    process.stdout.unwrap().read_to_string(&mut s).unwrap();
    s.pop(); // trim newline
    debug!("sig: {}", s);
    s
}

#[test]
fn test_parse_youtube() {
    let url = parse("ALZHF5UqnU4");
    println!("{:?}", url)
}
