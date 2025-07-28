use leptos::prelude::*;
use leptos_fluent::leptos_fluent;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use leptos_toaster::*;
use url::Url;

use crate::pages;

#[component]
fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        default_language: "en-gb",

        // Client side options
        // -------------------
        // Synchronize `<html lang="...">` attribute with
        // current active language.
        sync_html_tag_lang: true,
        // Synchronize `<html dir="...">` attribute with `"ltr"`,
        // `"rtl"` or `"auto"` depending on active language.
        sync_html_tag_dir: true,
        // Update language on URL parameter when changes.
        set_language_to_url_param: true,
        // Set initial language of user from URL in local storage.
        initial_language_from_url_param_to_local_storage: true,
        // Get initial language from local storage if not found
        // in an URL param.
        initial_language_from_local_storage: true,
        // Update language on local storage when changes.
        set_language_to_local_storage: true,
        // Get initial language from `navigator.languages`
        // if not found in local storage.
        initial_language_from_navigator: true,
        // Set initial language of user from navigator to local storage.
        initial_language_from_navigator_to_local_storage: true,
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let base_uri = document()
        .base_uri()
        .ok()
        .flatten()
        .and_then(|base| Url::parse(&base).ok())
        .map(|url| url.path().to_string())
        .unwrap_or_else(|| "/".to_string());

    view! {
        <I18nProvider>
            <Toaster position=ToasterPosition::BottomCenter></Toaster>
            <Router base=base_uri>
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=pages::LandingPage />
                </Routes>
            </Router>
        </I18nProvider>
    }
}
