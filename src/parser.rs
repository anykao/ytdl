use nom::types::CompleteStr;
use serde_json;
use std::fs;

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

pub fn parse_dash(vid: &str) -> String {
    // let body = reqwest::get(format!("https://www.youtube.com/watch?v={}", vid).as_str())?.text()?;
    // let filename = format!("{}.html", vid);
    // let body = fs::read_to_string("./pXwfDZLKYm8.html")?;
    // println!("{}", body);
    // fs::write(filename, body.as_str())?;
    let body = fs::read_to_string("./pXwfDZLKYm8.html").unwrap();
    let (_, json) = ytconfig(CompleteStr(body.as_str())).unwrap();
    let config: PlayerConfig = serde_json::from_str(&json).unwrap();
    // println!("{:#?}", config);

    // let mut resp = reqwest::get(format!("https://www.youtube.com/{}", config.assets.js).as_str())?;
    // let mut file = File::create("script.js").unwrap();
    // resp.copy_to(&mut file)?;
    String::new()
}
