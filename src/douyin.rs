use super::sanitize;
use super::VideoUrl;
use reqwest;
use serde_json::{self, Value};

named!(
    parse_data<&str, Vec<&str>>, 
    re_capture_static!(r"var data = \[(.+)\]")
);

pub fn parse(url: &str) -> VideoUrl {
    let body = reqwest::get(url).unwrap().text().unwrap();
    let (_, data) = parse_data(&body).unwrap();
    let v: Value = serde_json::from_str(data[1]).unwrap();
    debug!("{:#?}", v);
    if let Some(Value::String(url)) = v.pointer("/video/play_addr/url_list/0") {
        let short_id = v.pointer("/author/short_id").unwrap().as_str().unwrap();
        let nickname = v.pointer("/author/nickname").unwrap().as_str().unwrap();
        let goodname = sanitize(nickname, None);
        let aweme_id = v.pointer("/aweme_id").unwrap().as_str().unwrap();
        return VideoUrl::Direct(
            url.to_string(),
            format!("{}_{}_{}.mp4", short_id, goodname, aweme_id),
        );
    }
    VideoUrl::Failed
}

#[test]
fn test_parse_douyin() {
    let url = parse("https://www.tiktokv.com/i18n/share/video/6560042923969219841");
    println!("{:?}", url)
}
