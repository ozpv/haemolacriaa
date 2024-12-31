use leptos::prelude::*;
use leptos_icons::Icon;

use crate::components::buttons::LinkButton;
use crate::types::links::{SocialMediaInfo, Song};

#[component]
fn Appendix(social_media_info: Option<&'static SocialMediaInfo>) -> impl IntoView {
    view! {
        {social_media_info
            .map_or(
                ().into_any(),
                |info| view! {
                    <div class="flex justify-center">
                        <nav id="social-media-links">
                            <a class="flex text-text-dark my-2.5 p-2 rounded-sm transition-all ease-in duration-75 hover:bg-surface-dark" href=info.url>
                                <Icon icon={info.icon} {..} width="16" height="16"/>
                            </a>
                        </nav>
                    </div>
                }.into_any()
            )
        }
    }
}

#[component]
pub fn StreamingList(
    list_info: RwSignal<Song<&'static str>>,
    #[prop(optional)] appendix_social: Option<&'static SocialMediaInfo>,
) -> impl IntoView {
    view! {
        <div id=move || format!("{}-link-list", list_info.get().name)>
            <img
                class="block text-text-dark mx-auto pt-4 shadow-2xl shadow-crust-dark"
                src=move || list_info.get().image.cdn_path()
                width=move || list_info.get().image.width
                height=move || list_info.get().image.height
                alt=move || list_info.get().name
            />
            <h1 class="block text-text-dark text-center text-3xl font-bold font-sans pt-9">
                    {move || list_info.get().name}
            </h1>
            <h2 class="block text-subtext-dark text-center text-lg font-medium font-inter pt-4 pb-2.5">
                    {move || list_info.get().author}
            </h2>
            <div class="flex justify-center">
                <nav id="streaming-links">
                    {move || list_info.get()
                        .build_streaming_info()
                        .into_iter()
                        .map(|info| {
                            view! {
                                <LinkButton
                                    href=info.song_url
                                    id=format!("{}-link-button", info.platform_name)
                                >
                                    <Icon icon=info.platform_icon width="24" height="24" />
                                    <p class="pl-5">{info.platform_name}</p>
                                </LinkButton>
                            }
                        })
                        .collect_view()
                    }
                </nav>
            </div>
            {appendix_social
                .map_or(
                    ().into_any(),
                    |_| view! {
                        <Appendix social_media_info=appendix_social/>
                    }.into_any()
                )
            }
        </div>
    }
}
