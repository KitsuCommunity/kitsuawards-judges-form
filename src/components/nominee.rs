use leptos::{ev::MouseEvent, *};

use crate::icons::chevron::*;

#[component]
pub fn nominee<F, F2, F3>(
    cx: Scope,
    name: String,
    move_up: F,
    move_down: F2,
    remove_item: F3,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
    F2: Fn(MouseEvent) + 'static,
    F3: Fn(MouseEvent) + 'static,
{
    view! { cx,
        <li class="nominee">
            <button on:click=move_up class="nominee-up"><Chevron direction=Direction::Up container_class="chevron-smaller" /></button>
            <button on:click=move_down class="nominee-down"><Chevron direction=Direction::Down container_class="chevron-smaller" /></button>
            <h2>{name}</h2>
            <button on:click=remove_item class="nominee-remove">"x"</button>
        </li>
    }
}
