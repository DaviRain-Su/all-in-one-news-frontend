use crate::types::*;
use crate::REBASE_BASE__API_URL;
use dioxus::prelude::*;
use futures::future::join_all;

pub fn Aions(cx: Scope) -> Element {
    let aion = use_future(cx, (), |_| get_all_aions());

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
        author: _,
        time,
        id,
        introduce,
        tag: _,
        ..
    } = aion;
    let full_aion = use_ref(cx, || None);

    let url = url.as_str();

    let time = time.format("%D %l:%M %p");

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            border: "1px solid #4e4e4e", /* 更深的边框颜色 */


            onmouseenter: move |_event| {
                resolve_aion(full_aion.clone(), preview_state.clone(), *id)
            },

            div {
                font_size: "1.8rem", /* 更大的标题字体 */
                color: "#ffffff", /* 白色文字 */

                a {
                    href: url,
                    onfocus: move |_event| {
                        resolve_aion(full_aion.clone(), preview_state.clone(), *id)
                    },
                    text_decoration: "none", /* 去掉链接下划线 */
                    color: "#00aaff", /* Solana 蓝色链接颜色 */
                    transition: "color 0.3s ease", /* 平滑的颜色过渡效果 */
                    "{title}"
                }
            }

            div {
                display: "flex",
                flex_direction: "row",
                color: "#aaaaaa", /* 淡灰色文字 */

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

pub async fn get_aion_preview(id: i32) -> Result<AIonResponse, reqwest::Error> {
    let url = format!("{}/rebase/by_id?id={}", REBASE_BASE__API_URL, id);
    let result = reqwest::get(&url)
        .await?
        .json::<Vec<AIonResponse>>()
        .await?;

    assert!(result.len() == 1);

    Ok(result.first().unwrap().clone())
}

pub async fn get_aions(count: usize) -> Result<Vec<AIonResponse>, reqwest::Error> {
    let url = format!("{}/rebase/ids", REBASE_BASE__API_URL);

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
    let url = format!("{}/rebase/list_all", REBASE_BASE__API_URL);
    let result = reqwest::get(&url)
        .await?
        .json::<Vec<AIonResponse>>()
        .await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_get_rebase_dailys() {
        let result = get_aion_preview(4198).await.unwrap();
        println!("result = {:?}", result);
        let result = get_aions(10).await.unwrap();
        println!("result = {:?}", result);
    }
}
