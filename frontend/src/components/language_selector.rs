use std::collections::HashMap;

use leptos::prelude::*;
use leptos_fluent::I18n;

use super::DropdownMenu;

#[component]
pub fn LanguageSelector() -> impl IntoView {
    let i18n = expect_context::<I18n>();

    let options = HashMap::from(
        i18n.languages
            .iter()
            .map(|lang| {
                (
                    *lang,
                    format!("{}  {}", lang.flag.unwrap_or_default(), lang.name),
                )
            })
            .collect::<HashMap<_, _>>(),
    );

    let chosen_option = RwSignal::new(i18n.language.get_untracked());

    Effect::new(move || {
        i18n.language.set(&chosen_option.get());
    });

    view! { <DropdownMenu class:w-40 options=options chosen_option=chosen_option /> }
}
