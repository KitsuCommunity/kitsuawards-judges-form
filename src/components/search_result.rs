use leptos::{ev::MouseEvent, *};

use crate::{api::Anime, pages::nomination_list::Nominee};

#[component]
pub fn search_result<F>(
    cx: Scope,
    anime: Anime,
    insert_nominee: WriteSignal<Option<Nominee>>,
    clear_search: F,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    let (title, _) = create_signal(cx, anime.titles.canonical);
    let (id, _) = create_signal(cx, anime.id);

    let image = anime.poster_image.views[0].url.clone();

    let on_click = move |ev: MouseEvent| {
        ev.prevent_default();
        insert_nominee(Some((id.get(), title.get())));
        clear_search();
    };

    view! { cx,
        <li class="search-result">
            <button on:click=on_click>
                <img src=image alt="poster" />
                <h2>{title.get()}</h2>
            </button>
        </li>
    }
}
