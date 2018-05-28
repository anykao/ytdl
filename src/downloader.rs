use super::VideoUrl;
use rayon;
use reqwest;
use std::fs::{remove_file, File};
use std::process::Command;

pub fn download(url: VideoUrl) {
    match url {
        VideoUrl::Failed => {
            info!("Can't get download url");
        }
        VideoUrl::Direct { url, filename } => {
            let mut resp = reqwest::get(&url).unwrap();
            if resp.status().is_success() {
                let mut video = File::create(filename).unwrap();
                resp.copy_to(&mut video).unwrap();
            } else {
                info!("{:?}", resp.text());
                info!("{:?}", resp.status());
            }
        }
        VideoUrl::Dash {
            video_url,
            audio_url,
            filename: vid,
        } => {
            println!("[INFO] downloading {}.mkv", vid);
            let video_file = format!("{}_v", vid);
            let audio_file = format!("{}_a", vid);
            rayon::join(
                || {
                    // video
                    let video_file = format!("{}_v", vid);
                    let mut resp = reqwest::get(&video_url).unwrap();
                    if resp.status().is_success() {
                        let mut video = File::create(&video_file).unwrap();
                        resp.copy_to(&mut video).unwrap();
                    } else {
                        info!("{:?}", resp.text());
                        info!("{:?}", resp.status());
                    }
                },
                || {
                    // audio
                    let mut resp = reqwest::get(&audio_url).unwrap();
                    if resp.status().is_success() {
                        let mut audio = File::create(&audio_file).unwrap();
                        resp.copy_to(&mut audio).unwrap();
                    } else {
                        info!("{:?}", resp.text());
                        info!("{:?}", resp.status());
                    }
                },
            );
            // debug!("{}, {}", video_url, audio_url);
            // debug!("{}, {}", &video_file, &audio_file);
            merge(&video_file, &audio_file, &vid);
        }
    }
}

fn merge(v: &str, a: &str, vid: &str) {
    println!("[INFO] merging {}.mkv", vid);
    let output = format!("{}.mkv", vid);
    match Command::new("ffmpeg")
        .args(&["-i", a, "-i", v, "-c", "copy", &output])
        .output()
    {
        Ok(_) => {
            remove_file(v).expect("unable delete");
            remove_file(a).expect("unable delete");
        }
        Err(e) => {
            println!("{}: {:?}", "failed to execute process", e);
        }
    }
    println!("[INFO] {} done.", vid);
}

#[test]
fn test_download_direct() {
    let url = VideoUrl::Direct{
        url: "https://aweme.snssdk.com/aweme/v1/playwm/?video_id=v0200ff10000bc57r6g7q8ifnjl8pjg0&line=1".to_string(), 
        filename: "85757099860.mp4".to_string(),
    };
    println!("{:?}", url);
    download(url);
}
