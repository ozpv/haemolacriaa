use leptos::*;
use leptos_icons::*;

use crate::components::buttons::LinkButton;
use crate::types::links::{SocialMediaInfo, SongInfo};

#[component]
fn Appendix(social_media_info: Option<&'static SocialMediaInfo>) -> impl IntoView {
    view! {
        {
            move || match social_media_info {
                Some(info) => {
                    view! {
                        <div class="flex justify-center">
                            <nav id="social-media-links">
                                <a class="flex text-white mt-[10px] mb-[10px] p-[8px] rounded-sm transition-all ease-in duration-75 hover:bg-gray-800" href=info.url><Icon icon=info.icon width="16" height="16"/></a>
                            </nav>
                        </div>
                    }.into_any()
                },
                None => { view!(<br/>) }.into_any()
            }
        }
    }
}

#[component]
pub fn StreamingList(
    song_info: &'static SongInfo,
    id: &'static str,
    #[prop(default = false)] appendix: bool,
    // only pass Some if previous bool is true
    #[prop(optional)] appendix_social: Option<&'static SocialMediaInfo>,
) -> impl IntoView {
    view! {
        <div id=id>
            <img class="block mx-auto pt-[16px] shadow-2xl" src=song_info.image.path width=song_info.image.width height=song_info.image.height alt=song_info.name/>
            <h1 class="block text-white text-center text-3xl font-bold font-sans pt-[36px]">{song_info.name}</h1>
            <h2 class="block text-white-800 text-center text-lg font-medium font-sans pt-[16px] pb-[10px]">{song_info.author}</h2>
            <div class="flex justify-center">
                <nav id="streaming-links">
                    {
                        song_info.build_streaming_info().into_iter().map(|item| {
                            view! {
                                <LinkButton class="text-white text-md font-sans pt-[20px] pb-[20px] w-80" href=item.song_url id=format!("{}{}", song_info.name, "-link-button")>
                                    <Icon icon=item.platform_icon width="24" height="24"/>
                                    <p class="pl-4">{item.platform_name}</p>
                                </LinkButton>
                            }
                        }).collect_view()
                    }
                </nav>
            </div>
            {
                move || if appendix && appendix_social != None {
                    view!(<Appendix social_media_info=appendix_social/>)
                } else {
                    ().into_view()
                }
            }
        </div>
    }
}
