use clap::Parser;
use rusty_ytdl::{RequestOptions, Video, VideoOptions};
use std::path::Path;
use tokio;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    link: String,

    /// The folder where the video will be saved
    #[arg(short, long, default_value = ".")]
    folder: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let video_options = VideoOptions {
        request_options: RequestOptions {
            proxy: Some(
                reqwest::Proxy::http("72.10.164.178:15979")
                    .unwrap()
                    .basic_auth("a", "b"),
            ),
            ..Default::default()
        },

        quality: rusty_ytdl::VideoQuality::Highest,
        filter: rusty_ytdl::VideoSearchOptions::VideoAudio,
        ..Default::default()
    };

    let video = Video::new_with_options(&args.link, video_options).unwrap();

    let video_info = video.get_info().await.unwrap();
    let video_title = video_info.video_details.title;

    let _stream = video.stream().await.unwrap();

    let folder = Path::new(&args.folder).join(format!("{}.mp4", video_title));

    video.download(folder).await.unwrap();
}
