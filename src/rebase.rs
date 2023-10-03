use crate::types::*;
use crate::REBASE_BASE__API_URL;
use dioxus::prelude::*;
use futures::future::join_all;

pub fn Aions(cx: Scope) -> Element {
    // let current_page = use_state(cx, || 0);
    // let items_per_page = 10; // 你可以根据需要调整每页显示的项数

    let aion = use_future(cx, (), |_| get_all_aions());
    // let aion = use_future(cx, (current_page.get(),), |(page,)| {
    // get_aions_page(page, 10)
    // });
    match aion.value() {
        Some(Ok(list)) => render! {
            div {

                for item in list {
                    AionListing { aion: item.clone() }
                }

                // div {
                //     display: "flex",
                //     justify_content: "center",
                //     margin_top: "1rem",
                //     button {
                //         onclick: move |_| current_page += 1,
                //         "Previous Page"
                //     },
                //     span {
                //         style: "0 1rem",
                //         format!("Page {}", current_page.get() + 1)
                //     },
                //     button {
                //         onclick: move |_| current_page -= 1,
                //         "Next Page"
                //     },
                // }
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
            border: "1px solid #ddd",
            border_radius: "8px",
            overflow: "hidden", /* 确保内容溢出时被隐藏 */


            onmouseenter: move |_event| {
                resolve_aion(full_aion.clone(), preview_state.clone(), *id)
            },

            div {
                padding: "1.2rem",
                background: "#e0e8f0", /* 淡蓝灰色调背景，柔和而不晃眼 */
                color: "#333", /* 深灰色文字 */
                border_bottom: "1px solid #888",

                a {
                    href: url,
                    onfocus: move |_event| {
                        resolve_aion(full_aion.clone(), preview_state.clone(), *id)
                    },
                    text_decoration: "none",
                    color: "#555", /* 深灰色链接颜色 */
                    transition: "color 0.3s ease",
                    "{title}"
                }
            }


            div {
                display: "flex",
                flex_direction: "row",
                color: "#aaaaaa", /* 淡灰色文字 */

                div {
                    padding: "0.5rem",
                    flex: "1", /* 占据剩余空间 */
                    overflow: "hidden", /* 内容溢出时隐藏 */
                    white_space: "nowrap", /* 防止文本换行 */
                    text_overflow: "ellipsis", /* 文本溢出时显示省略号 */
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

pub async fn get_aions_page(
    page: usize,
    items_per_page: usize,
) -> Result<Vec<AIonResponse>, reqwest::Error> {
    let url = format!(
        "{}/rebase/list?page={}&per_page={}",
        REBASE_BASE__API_URL,
        page * items_per_page,
        items_per_page
    );
    let aion_ids = reqwest::get(&url).await?.json::<Vec<i32>>().await?;

    let aion_futures = aion_ids.iter().map(|&aion_id| get_aion_preview(aion_id));
    Ok(join_all(aion_futures)
        .await
        .into_iter()
        .filter_map(|aion| aion.ok())
        .collect())
}
