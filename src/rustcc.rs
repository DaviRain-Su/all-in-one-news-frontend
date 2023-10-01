#![allow(non_snake_case)]
use crate::types::*;
use crate::REBASE_BASE__API_URL;
use dioxus::prelude::*;
use futures::future::join_all;

pub fn Aions(cx: Scope) -> Element {
    let aion = use_future(cx, (), |_| get_latest_aions());

    match aion.value() {
        Some(Ok(list)) => render! {
            div {
                for item in list {
                    AionListing { aion: item.clone() }
                }
            }
        },
        Some(Err(err)) => render! {"An error occurred while fetching stories {err}"},
        None => render! {"Loading items"},
    }
}

async fn resolve_aion(
    full_story: UseRef<Option<AIonResponse>>,
    preview_state: UseSharedState<PreviewState>,
    story_id: i32,
) {
    if let Some(cached) = &*full_story.read() {
        *preview_state.write() = PreviewState::Loaded(cached.clone());
        return;
    }

    *preview_state.write() = PreviewState::Loading;
    if let Ok(story) = get_aion_preview(story_id).await {
        *preview_state.write() = PreviewState::Loaded(story.clone());
        *full_story.write() = Some(story);
    }
}

#[inline_props]
fn AionListing(cx: Scope, aion: AIonResponse) -> Element {
    let preview_state = use_shared_state::<PreviewState>(cx).unwrap();
    let AIonResponse {
        title,
        url,
        author: by,
        time,
        id,
        introduce,
        tag,
        ..
    } = aion;
    let full_aion = use_ref(cx, || None);

    let url = url.as_str();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");

    let time = time.format("%D %l:%M %p");

    cx.render(rsx! {
        div {
            padding: "0.5rem",
            position: "relative",
            onmouseenter: move |_event| {
                resolve_aion(full_aion.clone(), preview_state.clone(), *id)
            },
            div {
                font_size: "1.5rem",
                a {
                    href: url,
                    onfocus: move |_event| {
                        resolve_aion(full_aion.clone(), preview_state.clone(), *id)
                    },
                    "{title}"
                }
                a {
                    color: "gray",
                    href: url,
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                color: "gray",
                div {
                    padding_left: "0.5rem",
                    "{introduce}"
                }
                div {
                    padding_left: "0.5rem",
                    "by {by}"
                }
                div {
                    padding_left: "0.5rem",
                    "{time}"
                }
            }
            // div {
            //     padding: "0.5rem",
            //     display: "flex", // 行显示
            //     color: "red",
            //     div {
            //         display: "flex",
            //         flex_direction: "column",
            //         margin: "0.2rem",
            //         "Tag: "
            //     }
            //     for tg in tag {
            //         div {
            //             display: "flex",
            //             flex_direction: "column",
            //             margin: "0.2rem",
            //             " {tg}"
            //         }
            //     }
            // }
        }
    })
}

#[derive(Clone, Debug)]
pub enum PreviewState {
    Unset,
    Loading,
    Loaded(AIonResponse),
}

pub async fn get_aion_preview(id: i32) -> Result<AIonResponse, reqwest::Error> {
    let url = format!("{}/rustcc/by_id?id={}", REBASE_BASE__API_URL, id);
    let result = reqwest::get(&url)
        .await?
        .json::<Vec<AIonResponse>>()
        .await?;

    assert!(result.len() == 1);

    Ok(result.first().unwrap().clone())
}

pub async fn get_aions(count: usize) -> Result<Vec<AIonResponse>, reqwest::Error> {
    let url = format!("{}/rustcc/ids", REBASE_BASE__API_URL);

    let aion_ids = &reqwest::get(&url).await?.json::<Vec<i32>>().await?[..count];

    let aion_futures = aion_ids[..usize::min(aion_ids.len(), count)]
        .iter()
        .map(|&aion_id| get_aion_preview(aion_id));
    Ok(join_all(aion_futures)
        .await
        .into_iter()
        .filter_map(|aion| aion.ok())
        .collect())
}

pub async fn get_all_aions() -> Result<Vec<AIonResponse>, reqwest::Error> {
    let url = format!("{}/rustcc/list_all", REBASE_BASE__API_URL);
    let result = reqwest::get(&url)
        .await?
        .json::<Vec<AIonResponse>>()
        .await?;

    Ok(result)
}

pub async fn get_latest_aions() -> Result<Vec<AIonResponse>, reqwest::Error> {
    // let url = format!("{}/rustcc/latest", REBASE_BASE__API_URL);
    let url = "https://aion-qr8nz.ondigitalocean.app/rustcc/list_all";
    let result = reqwest::get(url).await?.json::<Vec<AIonResponse>>().await?;

    Ok(result)
}