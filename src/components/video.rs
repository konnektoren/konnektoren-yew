use regex::Regex;
use web_sys::console;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct VideoProps {
    pub src: String,
    #[prop_or_default]
    pub preview: Option<String>,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or(false)]
    pub autoplay: bool,
}

#[derive(Debug, PartialEq)]
enum VideoType {
    YouTube(String),
    Regular,
}

fn get_video_type(src: &str) -> VideoType {
    let youtube_patterns = [
        r"(?:https?:\/\/)?(?:www\.)?youtube\.com\/watch\?v=([^&\s]+)",
        r"(?:https?:\/\/)?(?:www\.)?youtu\.be\/([^&\s]+)",
        r"(?:https?:\/\/)?(?:www\.)?youtube\.com\/embed\/([^&\s]+)",
    ];

    for pattern in youtube_patterns.iter() {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(captures) = re.captures(src) {
                if let Some(video_id) = captures.get(1) {
                    return VideoType::YouTube(video_id.as_str().to_string());
                }
            }
        }
    }

    VideoType::Regular
}

fn get_youtube_preview(video_id: &str) -> String {
    format!("https://img.youtube.com/vi/{}/hqdefault.jpg", video_id)
}

#[function_component(VideoComponent)]
pub fn video(props: &VideoProps) -> Html {
    let playing = use_state(|| props.autoplay);
    let video_type = get_video_type(&props.src);

    let toggle_play = {
        let playing = playing.clone();
        Callback::from(move |_| {
            playing.set(true);
            console::log_1(&"Video Play button clicked!".into());
        })
    };

    let video_class = classes!("video", if *playing { "video--playing" } else { "" });

    let preview_class = classes!(
        "video__preview",
        if !*playing {
            "" // Preview is visible
        } else {
            "video__preview--hidden" // Preview is hidden
        }
    );

    let iframe_class = classes!("video__iframe");
    let player_class = classes!("video__player");

    let preview_content = {
        let preview = match (&props.preview, &video_type) {
            (Some(preview), _) => {
                if preview.starts_with("http") {
                    html! {
                        <img class="video__preview-image" src={preview.clone()} alt="Video preview" />
                    }
                } else {
                    html! {
                        <span class="video__preview-emoji">{preview.clone()}</span>
                    }
                }
            }
            (None, VideoType::YouTube(video_id)) => html! {
                <img
                    class="video__preview-image"
                    src={get_youtube_preview(video_id)}
                    alt="Video preview"
                />
            },
            _ => html! {
                <span class="video__preview-emoji">{"🎥"}</span>
            },
        };

        html! {
            <>
                {preview}
                <div class="video__play-button">
                    <i class="fas fa-play"></i>
                </div>
            </>
        }
    };

    let video_content = match video_type {
        VideoType::YouTube(video_id) => {
            let src = format!(
                "https://www.youtube.com/embed/{}?autoplay={}&rel=0",
                video_id,
                if *playing { "1" } else { "0" }
            );

            html! {
                <iframe
                    class={iframe_class}
                    src={src}
                    title={props.title.clone().unwrap_or_else(|| "YouTube video player".to_string())}
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                    allowfullscreen=true
                />
            }
        }
        VideoType::Regular => {
            html! {
                <video
                    class={player_class}
                    src={props.src.clone()}
                    controls=true
                    autoplay={*playing}
                >
                    <source src={props.src.clone()} type="video/mp4" />
                    {"Your browser does not support the video tag."}
                </video>
            }
        }
    };

    html! {
        <div class={video_class} onclick={toggle_play}>
            <div class={preview_class}>
                {preview_content}
            </div>
            {video_content}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        VideoComponent,
        VideoProps {
            src: "https://www.youtube.com/watch?v=9XEQldT2Ick".to_string(),
            preview: None,
            title: Some("How to Learn ANY Language".to_string()),
            autoplay: false,
        },
        (
            "with preview image",
            VideoProps {
                src: "https://www.youtube.com/watch?v=9XEQldT2Ick".to_string(),
                preview: Some(
                    "https://img.youtube.com/vi/9XEQldT2Ick/maxresdefault.jpg".to_string()
                ),
                title: Some("Language Learning Video with custom preview".to_string()),
                autoplay: false,
            }
        ),
        (
            "with emoji preview",
            VideoProps {
                src: "https://www.youtube.com/watch?v=9XEQldT2Ick".to_string(),
                preview: Some("🗣️".to_string()),
                title: Some("Language Learning Video with emoji preview".to_string()),
                autoplay: false,
            }
        ),
        (
            "with autoplay",
            VideoProps {
                src: "https://www.youtube.com/watch?v=9XEQldT2Ick".to_string(),
                preview: None,
                title: Some("Language Learning Video with autoplay".to_string()),
                autoplay: true,
            }
        ),
        (
            "regular video",
            VideoProps {
                src: "https://www.w3schools.com/tags/mov_bbb.mp4".to_string(),
                preview: Some("🐰".to_string()),
                title: Some("Big Buck Bunny with emoji preview".to_string()),
                autoplay: false,
            }
        )
    );
}
