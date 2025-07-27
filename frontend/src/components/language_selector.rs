use leptos::prelude::*;
use leptos_fluent::I18n;
use leptos_use::on_click_outside;

#[component]
pub fn LanguageSelector() -> impl IntoView {
    let node_ref = NodeRef::new();
    let is_open = RwSignal::new(false);

    let toggle = move |_| {
        is_open.update(|open| *open = !*open);
    };

    let _ = on_click_outside(node_ref, move |_| {
        is_open.set(false);
    });

    let i18n = expect_context::<I18n>();

    let select_language = move |lang| {
        i18n.language.set(lang);
        is_open.set(false);
    };

    view! {
        <style>
            ".dropdown-transition {
            opacity: 0;
            transform: scaleY(0.5);
            transform-origin: top;
            transition: all 150ms ease-out;
            pointer-events: none;
            }
            
            .dropdown-transition.open {
            opacity: 1;
            transform: scaleY(1);
            pointer-events: auto;
            }
            
            ul::-webkit-scrollbar {
            width: 8px;
            }
            
            ul::-webkit-scrollbar-track {
            background: #f3f4f6;
            border-radius: 4px;
            }
            
            ul::-webkit-scrollbar-thumb {
            background-color: #d1d5db;
            border-radius: 4px;
            border: 2px solid #f3f4f6;
            }
            
            ul {
            scrollbar-width: thin;
            scrollbar-color: #d1d5db #f3f4f6;
            }
            
            ul::-webkit-scrollbar-thumb:hover {
            background-color: #9ca3af;
            }
            "
        </style>
        <div class="relative w-60 z-20">
            <button
                on:click=toggle
                class="w-full text-left px-4 py-2 rounded-md text-gray-700 bg-gray-200 shadow-md border border-gray-300 hover:bg-gray-300 focus:outline-none"
            >
                {move || i18n.language.get().name}
                <span class="float-right">"▼"</span>
            </button>

            <ul
                class=move || {
                    format!(
                        "dropdown-transition absolute mt-1 w-full rounded-md bg-white border border-gray-300 shadow-lg max-h-80 overflow-auto {}",
                        if is_open.get() { "open" } else { "" },
                    )
                }
                node_ref=node_ref
            >
                {i18n
                    .languages
                    .iter()
                    .map(|lang| {
                        view! {
                            <li
                                on:click={
                                    let select_language = select_language.clone();
                                    move |_| select_language(lang)
                                }
                                class="cursor-pointer px-4 py-2 hover:bg-gray-100 text-gray-700"
                            >
                                {lang.name}
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
