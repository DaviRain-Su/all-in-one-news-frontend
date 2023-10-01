use crate::types::*;
use crate::REBASE_BASE__API_URL;
use dioxus::prelude::*;

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
    hash: String,
) {
    if let Some(cached) = &*full_story.read() {
        *preview_state.write() = PreviewState::Loaded(cached.clone());
        return;
    }

    *preview_state.write() = PreviewState::Loading;
    if let Ok(story) = get_aion_preview(hash).await {
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
        author: _,
        time,
        introduce,
        tag: _,
        hash,
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
                resolve_aion(full_aion.clone(), preview_state.clone(), hash.clone())
            },
            div {
                font_size: "1.5rem",
                a {
                    href: url,
                    onfocus: move |_event| {
                        resolve_aion(full_aion.clone(), preview_state.clone(), hash.clone())
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
                    "{time}"
                }
            }
        }
    })
}

#[derive(Clone, Debug)]
pub enum PreviewState {
    Unset,
    Loading,
    Loaded(AIonResponse),
}

pub async fn get_aion_preview(hash: String) -> Result<AIonResponse, reqwest::Error> {
    let url = format!("{}/rustcc/by_hash?hash={}", REBASE_BASE__API_URL, hash);
    let result = reqwest::get(&url)
        .await?
        .json::<Vec<AIonResponse>>()
        .await?;

    assert!(result.len() == 1);

    Ok(result.first().unwrap().clone())
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
    let url = format!("{}/rustcc/latest", REBASE_BASE__API_URL);
    let result = reqwest::get(url).await?.json::<Vec<AIonResponse>>().await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_aions() {
        let aions = get_latest_aions().await.unwrap();
        println!("aions = {:?}", aions);
    }

    #[tokio::test]
    async fn test_get_aion_preview() {
        let aion = get_aion_preview(
            "637e364663a7265da4a0c665528c65adff6054195af9d285bb4b62b2d6971591".to_string(),
        )
        .await
        .unwrap();
        println!("aion = {:?}", aion);
    }
}
