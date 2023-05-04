use crate::api::{get_anime, Anime};
use crate::components::SearchResult::*;
use crate::pages::NominationList::Nominee;

use leptos::{leptos_dom::helpers::debounce, leptos_dom::helpers::*, *};
use log::info;
use std::time::Duration;

#[component]
pub fn Search(cx: Scope, insert_nominee: WriteSignal<Option<Nominee>>) -> impl IntoView {
    // Creates a reactive value to update the button
    let (text, set_text) = create_signal(cx, String::new());
    let (data, set_data) = create_signal::<Vec<Anime>>(cx, vec![]);

    let get_anime = create_action(cx, |title: &String| {
        let title = title.to_owned();
        info!("get_anime: {}", title);

        async move { get_anime(title).await }
    });

    let mut on_type = debounce(cx, Duration::from_millis(800), move |_: &()| {
        info!("on_type: {}", text.get());

        set_data.set(vec![]);

        if !text.get().is_empty() {
            get_anime.dispatch(text.get());
        }
    });

    let clear_search = move || {
        set_data(vec![]);
        set_text(String::new());
    };

    let fetched = get_anime.value();
    let fetching = get_anime.pending();

    create_effect(cx, move |_| {
        fetched.track();
        // info!("{:#?}", fetched());
        if let Some(anime) = fetched() {
            set_data.set(anime);
        }
    });

    view! { cx,
        <input
            class="search-input"
            on:input=move |ev| {
                let title = event_target_value(&ev);
                set_text(title);
                on_type(&());
            }
            prop:value=text
            value=text
            type="search"
            placeholder="Search Kitsu ..."
        ></input>
        <p>{move || if fetching() { "loading ..." } else { "" } }</p>

        <Show
            when=move || !data.get().is_empty()
            fallback=|_cx| view! {cx, <></> }
        >
            <ul class="search-results-list">
                { move || {
                    let list = data
                        .get()
                        .into_iter()
                        .map(|anime| view! {cx, <SearchResult anime=anime insert_nominee=insert_nominee clear_search=clear_search />})
                        .collect::<Vec<_>>();

                    view! {cx, {list} }
                }}
            </ul>
        </Show>
    }
}
