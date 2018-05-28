use super::sanitize;
use super::VideoUrl;
use reqwest;
use reqwest::header::UserAgent;
use serde_json::{self, Value};

named!(
    parse_data<&str, Vec<&str>>, 
    re_capture_static!(r"var data = \[(.+)\]")
);

pub fn list(uid: &str) -> Vec<VideoUrl> {
    let mut ret = Vec::new();
    let url = &format!(
        "https://www.amemv.com/aweme/v1/aweme/post/?user_id={}&max_cursor=0&count=20",
        uid
    );
    let client = reqwest::Client::new();
    let mut res = client.get(url)
        .header(UserAgent::new("Mozilla/5.0 (Linux; U; Android 5.1.1; zh-cn; MI 4S Build/LMY47V) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/53.0.2785.146 Mobile Safari/537.36 XiaoMi/MiuiBrowser/9.1.3"))
        .send()
        .unwrap();
    if res.status().is_success() {
        let body = res.text().unwrap();
        let v: Value = serde_json::from_str(&body).unwrap();
        println!("status_code: {:?}", v["status_code"]);
        if let Some(aweme_list) = v["aweme_list"].as_array() {
            for v in aweme_list {
                let url = get_video_url(v);
                ret.push(url);
            }
        }
    }
    ret
}

pub fn parse(url: &str) -> VideoUrl {
    let client = reqwest::Client::new();
    let body = client.get(url)
        .header(UserAgent::new("Mozilla/5.0 (Linux; U; Android 5.1.1; zh-cn; MI 4S Build/LMY47V) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/53.0.2785.146 Mobile Safari/537.36 XiaoMi/MiuiBrowser/9.1.3"))
        .send()
        .unwrap()
        .text()
        .unwrap();
    let (_, data) = parse_data(&body).unwrap();
    let v: Value = serde_json::from_str(data[1]).unwrap();
    debug!("{:#?}", v);
    get_video_url(&v)
}

fn get_video_url(v: &Value) -> VideoUrl {
    if let Some(Value::String(url)) = v.pointer("/video/play_addr/url_list/0") {
        let short_id = v.pointer("/author/short_id").unwrap().as_str().unwrap();
        let nickname = v.pointer("/author/nickname").unwrap().as_str().unwrap();
        let goodname = sanitize(nickname, None);
        let aweme_id = v.pointer("/aweme_id").unwrap().as_str().unwrap();
        let new_url = url.replace(
            "https://api.tiktokv.com/aweme/v1/playwm",
            "https://api.tiktokv.com/aweme/v1/play",
        );
        VideoUrl::Direct {
            url: new_url,
            filename: format!("{}_{}_{}.mp4", short_id, goodname, aweme_id),
        }
    } else {
        VideoUrl::Failed
    }
}

#[test]
fn test_parse_douyin() {
    let url = parse("https://www.tiktokv.com/i18n/share/video/6560042923969219841");
    println!("{:?}", url)
}

#[test]
fn test_list() {
    let body = list("daijiali218");
    println!("{:?}", body)
}
