use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Welcome!"</h1>
        <p>"test"</p>
        <A href="nominations">"Nominations"</A>
    }
}
