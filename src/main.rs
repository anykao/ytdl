#![cfg_attr(feature = "flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature = "flame_it", plugin(flamer))]
#[macro_use]
extern crate structopt;
extern crate url;
#[macro_use]
extern crate log;
extern crate fern;
#[cfg(feature = "flame_it")]
extern crate flame;
extern crate ytdl;

// mod decipher;
// mod downloader;
mod logger;
// mod parser;

// use downloader::download;
// use parser::parse;
use std::collections::HashMap;
use structopt::StructOpt;
use url::Url;

use ytdl::downloader::download;
use ytdl::parse;
use ytdl::Provider;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "ytdl")]
enum Opt {
    #[structopt(name = "download")]
    /// download video
    Download { url_or_id: String },
    #[structopt(name = "list")]
    /// list douyin videos by user_id
    List { url_or_id: String },
}

fn main() {
    // ::std::env::set_var("RUST_BACKTRACE", "full");
    logger::init();
    let opt = Opt::from_args();
    match opt {
        Opt::Download { url_or_id } => {
            if let Some(ref p) = get_info(url_or_id) {
                let video_url = parse(p);
                #[cfg_attr(feature = "flame_it", flame)]
                download(video_url);
            } else {
                debug!("unknown provider")
            }
        }
        Opt::List { url_or_id } => {
            println!("{}", url_or_id);
        }
    }
    // Dump the report to disk
    #[cfg(feature = "flame_it")]
    flame::dump_html(&mut ::std::fs::File::create("flame-graph.html").unwrap()).unwrap();
}

fn get_info(url_or_id: String) -> Option<Provider> {
    if url_or_id.starts_with("http") {
        let url = Url::parse(&url_or_id).unwrap();
        let m: HashMap<_, _> = url.query_pairs().into_owned().collect();
        match url.host_str() {
            Some("youtube") => Some(Provider::Youtube(m["v"].clone())),
            Some("www.douyin.com") | Some("www.tiktokv.com") => Some(Provider::Douyin(url_or_id)),
            h => {
                println!("{:?}", h);
                None
            }
        }
    } else {
        Some(Provider::Youtube(url_or_id))
    }
}

// https://www.tiktokv.com/i18n/share/video/6560042923969219841
