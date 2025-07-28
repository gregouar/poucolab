use std::collections::HashMap;

use leptos::prelude::*;
use leptos_use::on_click_outside;

// #[component]
// pub fn DropdownMenu(options: Vec<(String, String)>) -> impl IntoView {
//     let open = SignalRw::new(false);

//     let toggle = move |_| open.update(|v| *v = !*v);

//     view! {
//         <div class="relative inline-block text-left">
//             <button
//                 class="inline-flex justify-center w-full px-3 py-2 text-sm font-medium text-white bg-gray-800 rounded-md hover:bg-gray-700 transition"
//                 aria-haspopup="true"
//                 on:click=toggle
//             >
//                 <svg class="ml-2 h-4 w-4 fill-current" viewBox="0 0 20 20">
//                     <path d="M5.25 7.5L10 12.25L14.75 7.5H5.25Z" />
//                 </svg>
//             </button>

//             <div
//                 class="origin-top-right absolute right-0 mt-2 w-40 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 z-50 transition-all duration-150 ease-out"
//                 class:hidden=move || !open.get()
//             >
//                 <div class="py-1 text-gray-900 text-sm">{children()}</div>
//             </div>
//         </div>
//     }
// }

#[component]
pub fn DropdownMenu<T>(
    options: HashMap<T, &'static str>,
    chosen_option: RwSignal<T>,
) -> impl IntoView
where
    T: Clone + std::hash::Hash + Eq + Send + Sync + 'static,
{
    let node_ref = NodeRef::new();

    let is_open = RwSignal::new(false);

    let toggle = move |_| {
        is_open.update(|open| *open = !*open);
    };

    let _ = on_click_outside(node_ref, move |_| {
        is_open.set(false);
    });

    let select_option = {
        move |opt| {
            is_open.set(false);
            chosen_option.set(opt);
        }
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
            background: #1f1f1f;
            border-radius: 4px;
            }
            
            ul::-webkit-scrollbar-thumb {
            background-color: #525252;
            border-radius: 4px;
            border: 2px solid #1f1f1f;
            }
            
            ul {
            scrollbar-width: thin;
            scrollbar-color: #525252 #1f1f1f;
            }
            
            ul::-webkit-scrollbar-thumb:hover {
            background-color: #737373;
            }
            "
        </style>
        <div class="relative w-60 z-20">
            <button
                on:click=toggle
                class="w-full text-left px-4 py-2 rounded-md text-white bg-gradient-to-t from-zinc-900 to-zinc-800 shadow-md border border-zinc-950 hover:from-zinc-800 hover:to-zinc-700 focus:outline-none"
            >
                {
                    let options = options.clone();
                    move || {
                        options
                            .get(&chosen_option.get())
                            .cloned()
                            .unwrap_or("Select an option".into())
                    }
                }
                <span class="float-right">"▼"</span>
            </button>

            <ul
                class=move || {
                    format!(
                        "dropdown-transition absolute mt-1 w-full rounded-md bg-zinc-800 border border-zinc-950 shadow-lg max-h-80 overflow-auto {}",
                        if is_open.get() { "open" } else { "" },
                    )
                }
                node_ref=node_ref
            >
                {options
                    .into_iter()
                    .map(|(opt, text)| {
                        view! {
                            <li
                                on:click=move |_| select_option(opt.clone())
                                class="cursor-pointer px-4 py-2 hover:bg-zinc-700 text-white"
                            >
                                {text}
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
