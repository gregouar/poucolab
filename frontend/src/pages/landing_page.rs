use leptos::{html::*, prelude::*};
use leptos_fluent::move_tr;

use crate::components::LanguageSelector;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <main class="min-h-screen bg-gradient-to-br from-slate-950 to-gray-900 text-white font-sans">
            <header class="w-full py-6 px-6 flex justify-between items-center border-b border-gray-800">
                <h1 class="text-2xl font-semibold tracking-tight">"PouCoLab"</h1>
                <nav class="space-x-6">
                    <a href="/features" class="hover:text-blue-400">
                        "Features"
                    </a>
                    <a href="/pricing" class="hover:text-blue-400">
                        "Pricing"
                    </a>
                    <a href="/login" class="hover:text-blue-400">
                        "Sign In"
                    </a>
                    <a
                        href="/signup"
                        class="ml-4 px-4 py-2 bg-blue-600 rounded-md hover:bg-blue-500 transition"
                    >
                        "Sign Up"
                    </a>

                    <LanguageSelector />
                </nav>
            </header>

            <section class="text-center py-24 px-4 max-w-4xl mx-auto">
                <h2 class="text-4xl md:text-6xl font-bold tracking-tight leading-tight mb-6">
                    {move_tr!("landing-tagline")}
                </h2>
                <p class="text-lg md:text-2xl text-gray-300 mb-10">
                    {move_tr!("landing-description")}
                </p>
                <div class="flex justify-center space-x-4">
                    <a
                        href="/signup"
                        class="px-6 py-3 rounded-md bg-blue-600 hover:bg-blue-500 transition text-lg"
                    >
                        {move_tr!("cta-signup")}
                    </a>
                    <a
                        href="/demo"
                        class="px-6 py-3 rounded-md border border-gray-500 hover:bg-gray-800 transition text-lg"
                    >
                        {move_tr!("cta-demo")}
                    </a>
                </div>
            </section>

            <section class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto px-6 py-20 border-t border-gray-800">
                <div>
                    <h3 class="text-xl font-semibold mb-2">"Templated Measurements"</h3>
                    <p class="text-gray-400">
                        "Design reusable data models for anatomical features, bones, or specimens."
                    </p>
                </div>
                <div>
                    <h3 class="text-xl font-semibold mb-2">"Team Collaboration"</h3>
                    <p class="text-gray-400">
                        "Invite contributors, manage access, and track edits with full lineage."
                    </p>
                </div>
                <div>
                    <h3 class="text-xl font-semibold mb-2">"Offline + Sync"</h3>
                    <p class="text-gray-400">
                        "Work in disconnected field environments, then sync when online."
                    </p>
                </div>
            </section>

            <footer class="py-12 text-center text-gray-600 text-sm border-t border-gray-800">
                "© 2025 Grégoire Naisse. All rights reserved."
            </footer>
        </main>
    }
}
