use leptos::prelude::*;
use leptos_icons::*;

use crate::components::buttons::LinkButton;
use crate::types::links::{SocialMediaInfo, Song};

#[component]
fn Appendix(social_media_info: Option<&'static SocialMediaInfo>) -> impl IntoView {
    view! {
        {
            social_media_info
                .map_or(().into_any(), |info| view! {
                    <div class="flex justify-center">
                        <nav id="social-media-links">
                            <a class="flex text-white mt-[10px] mb-[10px] p-[8px] rounded-sm transition-all ease-in duration-75 hover:bg-gray-800" href=info.url><Icon icon=info.icon width="16" height="16"/></a>
                        </nav>
                    </div>
                }.into_any())
        }
    }
}

#[component]
pub fn StreamingList(
    list_info: Song,
    #[prop(optional)] appendix_social: Option<&'static SocialMediaInfo>,
) -> impl IntoView {
    view! {
        <div id=format!("{}-link-list", list_info.name)>
            <img class="block mx-auto pt-[16px] shadow-2xl" src=list_info.image.path.clone() width=list_info.image.width.clone().unwrap_or("400px".to_owned()) height=list_info.image.height.clone().unwrap_or("400px".to_owned()) alt=list_info.name.clone()/>
            <h1 class="block text-white text-center text-3xl font-bold font-sans pt-[36px]">{list_info.name.clone()}</h1>
            <h2 class="block text-white-800 text-center text-lg font-medium font-sans pt-[16px] pb-[10px]">{list_info.author.clone()}</h2>
            <div class="flex justify-center">
                <nav id="streaming-links">
                    {list_info
                        .clone()
                        .build_streaming_info()
                        .into_iter()
                        .map(|item| {
                            item.song_id.map_or(().into_any(), |(platform_name, song_url)| {
                                view! {
                                    <LinkButton class="text-white text-md font-sans pt-[20px] pb-[20px] w-80 hover:scale-105" href=song_url id=format!("{}-link-button", platform_name)>
                                        <Icon icon=item.platform_icon width="24" height="24"/>
                                        <p class="pl-4">{platform_name}</p>
                                    </LinkButton>
                                }.into_any()
                            })
                        }).collect_view()
                    }
                </nav>
            </div>
            {appendix_social
                .map_or(().into_any(), |_| view! {
                        <Appendix social_media_info=appendix_social/>
                }.into_any())
            }
        </div>
    }
}
