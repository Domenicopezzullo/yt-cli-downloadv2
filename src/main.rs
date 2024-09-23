use tokio;
use youtube_dl::YoutubeDl;
use std::env;

#[tokio::main]
async fn main () {
    if let Some(url) = env::args().nth(1) {
        if let Some(folder) = env::args().nth(2) {
            let _video = YoutubeDl::new(&url).download_to(&folder);
        } else {
            let _video = YoutubeDl::new(&url).download_to(".");
        }
    } else {
        println!("URL Needed to work!");
    }
}
