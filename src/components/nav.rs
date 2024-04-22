use crate::config::NAV_ITEMS;
use icondata as ico;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Nav() -> impl IntoView {
    let (active, activity) = create_signal(false);
    let toggle_active = move |_| activity.update(|status| *status = !*status);

    let active_icon = Signal::derive(move || {
        if active.get() {
            ico::BsXLg
        } else {
            ico::BsList
        }
    });

    let hidden_status = move || !active.get();

    view! {
        <nav class="bg-gray-900 border-gray-200">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                <span class="flex items-center text-xl text-white font-sans px-[20px]">{env!("CARGO_PKG_NAME")}</span>
                <button
                    on:click=toggle_active
                    class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 md:hidden"
                >
                    <Icon
                        icon=active_icon
                        width="32px"
                        height="32px"
                    />
                </button>
                <div
                    class="w-full md:block md:w-auto hidden"
                    class:hidden=hidden_status
                    id="navbar-default"
                >
                    <ul
                        class="font-sans font-medium flex flex-col p-[16px] md:p-0 mt-4 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-gray-900"
                    >
                        {
                            NAV_ITEMS.iter().map(|item| {
                                view! {
                                    <li class="p-[8px]">
                                        <a
                                            href=item.path
                                            class="text-white py-[10px] px-[10px] hover:bg-gray-800 md:hover:text-blue-900 md:hover:bg-transparent"
                                        >
                                            {item.name}
                                        </a>
                                    </li>
                                }
                            }).collect_view()
                        }
                    </ul>
                </div>
            </div>
        </nav>
    }
}
