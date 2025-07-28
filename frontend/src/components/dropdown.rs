use std::collections::HashMap;

use leptos::{logging::log, prelude::*};
use leptos_use::on_click_outside;

#[component]
pub fn DropdownMenu<T>(options: HashMap<T, String>, chosen_option: RwSignal<T>) -> impl IntoView
where
    T: std::fmt::Debug + Clone + std::hash::Hash + Eq + Send + Sync + 'static,
{
    let node_ref = NodeRef::new();
    let is_open = RwSignal::new(false);

    let toggle = move |_| is_open.update(|open| *open = !*open);
    let _ = on_click_outside(node_ref, move |_| is_open.set(false));
    let select_option = move |opt| {
        is_open.set(false);
        chosen_option.set(opt);
    };

    view! {
        <style>
            ".dropdown-transition {
                opacity: 0;
                transform: scaleY(0.95);
                transform-origin: top;
                transition: all 180ms ease;
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
                background: #1e1e1e;
                border-radius: 4px;
            }
            
            ul::-webkit-scrollbar-thumb {
                background-color: #555;
                border-radius: 4px;
                border: 2px solid #1e1e1e;
            }
            
            ul {
                scrollbar-width: thin;
                scrollbar-color: #555 #1e1e1e;
            }
            
            ul::-webkit-scrollbar-thumb:hover {
                background-color: #777;
            }"
        </style>

        <div class="relative z-30 font-sans text-sm" node_ref=node_ref>
            <button
                on:click=toggle
                class="w-full text-left px-4 py-2 rounded-lg text-white bg-gradient-to-b from-slate-800 to-slate-700 border border-slate-600 shadow-inner hover:from-slate-700 hover:to-slate-600 transition-colors duration-150"
            >
                {
                    let options = options.clone();
                    move || {
                        log!("Chosen option: {:?}", chosen_option.get());
                        options
                            .get(&chosen_option.get())
                            .cloned()
                            .unwrap_or("Select an option".to_string())
                    }
                }
                <span class="float-right text-xs text-slate-400">"▼"</span>
            </button>

            <ul class=move || {
                format!(
                    "dropdown-transition absolute mt-2 w-full rounded-md bg-slate-800 border border-slate-600 shadow-xl max-h-72 overflow-auto backdrop-blur-sm ring-1 ring-black ring-opacity-20 {}",
                    if is_open.get() { "open" } else { "" },
                )
            }>
                {options
                    .into_iter()
                    .map(|(opt, text)| {
                        view! {
                            <li
                                on:click=move |_| select_option(opt.clone())
                                class="cursor-pointer px-4 py-2 text-slate-100 hover:bg-slate-700 hover:text-white transition-colors duration-100"
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
