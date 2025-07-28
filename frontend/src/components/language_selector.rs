use std::collections::HashMap;

use leptos::prelude::*;
use leptos_fluent::I18n;

use super::DropdownMenu;

#[component]
pub fn LanguageSelector() -> impl IntoView {
    let i18n = expect_context::<I18n>();

    let options = i18n
        .languages
        .iter()
        .map(|lang| {
            (
                lang.id,
                format!("{}  {}", lang.flag.unwrap_or_default(), lang.name),
            )
        })
        .collect::<HashMap<_, _>>();

    let chosen_option = RwSignal::new(i18n.language.get_untracked().id);

    Effect::new(move || {
        let chosen_language = chosen_option.get();
        if let Some(lang) = i18n.languages.iter().find(|l| l.id == chosen_language) {
            i18n.language.set(lang);
        }
    });

    view! { <DropdownMenu class:w-40 options=options chosen_option=chosen_option /> }
}
