use crate::api::{get_anime, Anime};

use leptos::{leptos_dom::helpers::debounce, *};
use log::info;
use std::time::Duration;

#[component]
pub fn Search(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (data, set_data) = create_signal::<Vec<Anime>>(cx, vec![]);
    let (debounced_text, set_debounced_text) = create_signal(cx, "".to_string());

    let get_anime = create_resource(cx, debounced_text, |title| async move {
        info!("{}", title);

        get_anime(title).await
    });

    let on_type = debounce(cx, Duration::from_secs(1), move |ev| {
        let target = event_target_value(&ev);

        if !target.is_empty() {
            set_debounced_text(target);
        }
    });

    create_effect(cx, move |_| {
        // info!("{:#?}", fetched());
        if let Some(anime) = get_anime.read(cx) {
            set_data.set(anime);
        }
    });

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <input on:input=on_type></input>
        // <p>{move || format!("input is {:#?}", input())}</p>
        // <For each=empty key=|val| val.1.clone() view=move |cx, val| view! {cx, <p>{val.0}</p> } />
        <ul>
            <Show
                when=move || !data.get().is_empty()
                fallback=|cx| view! {cx, <li></li> }
            >
                <For each=get_anime key=|anime| anime.id.clone() view=move |cx, anime| view! {cx,  <li>{anime.titles.canonical}</li> }/>
            </Show>
        </ul>
    }
}
