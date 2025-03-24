use chrono::{Datelike, NaiveDate};
use leptos::prelude::*;
use leptos_meta::Meta;

use crate::components::nav;
use crate::config::{CURRENT_SONG, OTHER_SONGS};
use crate::types::images::Image;

#[component]
fn SongCard(
    image: Image<&'static str>,
    author: &'static str,
    name: &'static str,
    publish_date: Option<NaiveDate>,
) -> impl IntoView {
    view! {
        <a class="content-none" href=format!("/releases/{name}")>
            <img src=image.cdn_path()
                width=image.width.unwrap_or("400")
                height=image.height.unwrap_or("400")
                class="text-text-dark transition duration-100 hover:scale-[103%]"
                alt=format!("{name} album art")
            />
            <h2 class="text-text-dark text-lg tracking-[2px] font-sans pt-2">{author}</h2>
            <p class=format!("text-text-dark{}", publish_date.map_or(" pb-2", |_| ""))>
                {name}
            </p>
            {
                if let Some(date) = publish_date {
                    let date =
                        format!("{}.{}.{}", date.month(), date.day(), date.year());

                    view! {
                        <p class="text-subtext-dark pb-2">{date}</p>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }
        </a>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <nav::Nav/>
        <Meta name="description" content="leave by haemolacriaa is out now" />
        <main class="main">
            <div class="flex flex-row justify-center mx-5">
                <ol>
                    <SongCard image=CURRENT_SONG.image
                        author=CURRENT_SONG.author
                        name=CURRENT_SONG.name
                        publish_date=CURRENT_SONG.publish_date
                    />
                    {
                        OTHER_SONGS
                            .iter()
                            .map(|song| view! {
                                <SongCard image=song.image
                                    author=song.author
                                    name=song.name
                                    publish_date=song.publish_date
                                />
                            })
                            .collect_view()
                    }
                </ol>
            </div>
        </main>
    }
}
