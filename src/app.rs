use crate::pages::home::*;
use crate::pages::nomination_list::*;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/kitsuawards_judges_form.css"/>

        // sets the document title
        <Title text="Kitsu Anime Awards Form"/>

        // content for this welcome page
        <Router>
            <main>
                <div id="content">
                <Routes>
                    <Route path="/nominations" view=|cx| view! { cx, <NominationList category="Best Anime".to_string() /> }/>
                    <Route path="" view=|cx| view! { cx, <Home />}/>
                </Routes>
                </div>
            </main>
        </Router>
    }
}
