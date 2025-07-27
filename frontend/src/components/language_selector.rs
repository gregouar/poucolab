use leptos::prelude::*;
use leptos_fluent::I18n;

#[component]
pub fn LanguageSelector() -> impl IntoView {
    // `expect_context::<leptos_fluent::I18n>()` to get the i18n context
    // `i18n.languages` exposes a static array with the available languages
    // `i18n.language.get()` to get the active language
    // `i18n.language.set(lang)` to set the active language

    let i18n = expect_context::<I18n>();

    view! {
        <fieldset>
            {move || {
                i18n.languages
                    .iter()
                    .map(|lang| {
                        view! {
                            <div>
                                <input
                                    type="radio"
                                    id=lang
                                    name="language"
                                    value=lang
                                    checked=&i18n.language.get() == lang
                                    on:click=move |_| i18n.language.set(lang)
                                />
                                <label for=lang>{lang.name}</label>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </fieldset>
    }
}
