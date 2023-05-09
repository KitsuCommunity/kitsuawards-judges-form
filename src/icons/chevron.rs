use leptos::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[component]
pub fn chevron<'a>(cx: Scope, direction: Direction, container_class: &'a str) -> impl IntoView {
    let class = match direction {
        Direction::Up => "chevron up",
        Direction::Down => "chevron down",
        Direction::Left => "chevron left",
        Direction::Right => "chevron right",
    };

    view! { cx,
        <div class=container_class>
            <svg class=class id="icon-right-chevron" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 120 120" xml:space="preserve">
                <polygon fill="#010101" points="102.296,59.957 42.264,119.99 25.605,103.34 85.639,43.299 "/>
                <polygon fill="#010101" points="85.74,76.71 25.715,16.653 42.373,0.011 102.391,60.067 "/>
            </svg>
        </div>
    }
}
