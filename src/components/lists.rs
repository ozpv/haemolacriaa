use leptos::*;
use leptos_icons::*;

use crate::components::buttons::LinkButton;
use crate::types::links::{SocialMediaInfo, SongInfo};

#[component]
fn Appendix(social_media_info: Option<&'static SocialMediaInfo>) -> impl IntoView {
    view! {
        {
            social_media_info
                .map_or(().into_view(), |info| {
                     view! {
                        <div class="flex justify-center">
                            <nav id="social-media-links">
                                <a class="flex text-white mt-[10px] mb-[10px] p-[8px] rounded-sm transition-all ease-in duration-75 hover:bg-gray-800" href=info.url><Icon icon=info.icon width="16" height="16"/></a>
                            </nav>
                        </div>
                    }.into_view()
                })
        }
    }
}

#[component]
pub fn StreamingList(
    list_info: ReadSignal<(&'static SongInfo, String)>,
    #[prop(optional)] 
    appendix_social: Option<&'static SocialMediaInfo>,
) -> impl IntoView {
    let song_info = move || list_info.get().0;
    let id = move || list_info.get().1;

    view! {
        <div id=id>
            <img class="block mx-auto pt-[16px] shadow-2xl" src=move || song_info().image.path width=move || song_info().image.width height=move || song_info().image.height alt=move || song_info().name/>
            <h1 class="block text-white text-center text-3xl font-bold font-sans pt-[36px]">{move || song_info().name}</h1>
            <h2 class="block text-white-800 text-center text-lg font-medium font-sans pt-[16px] pb-[10px]">{move || song_info().author}</h2>
            <div class="flex justify-center">
                <nav id="streaming-links">
                    {
                        move || song_info()
                            .build_streaming_info()
                            .into_iter()
                            .map(|item| {
                                item.song_id.map_or(().into_view(), |(platform_name, song_url)| {
                                    view! {
                                        <LinkButton class="text-white text-md font-sans pt-[20px] pb-[20px] w-80 hover:scale-105" href=song_url id=format!("{}-link-button", platform_name)>
                                            <Icon icon=item.platform_icon width="24" height="24"/>
                                            <p class="pl-4">{platform_name}</p>
                                        </LinkButton>
                                    }.into_view()
                                })
                            }
                            ).collect_view()
                    }
                </nav>
            </div>
            {
                appendix_social
                    .map_or(().into_view(), |_| {
                        view! {
                            <Appendix social_media_info=appendix_social/>
                        }
                    })
            }
        </div>
    }
}
