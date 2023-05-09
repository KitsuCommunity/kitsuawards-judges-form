use crate::{components::nominee::*, pages::search::*};

use leptos::*;
use log::info;

pub type Nominee = (String, String);

pub enum Move {
    Up,
    Down,
}

pub type MoveArgs<'a> = &'a Nominee;

fn test_data() -> Vec<Nominee> {
    vec![
        ("1".to_string(), "K-On!".to_string()),
        ("2".to_string(), "KiraKira Precure".to_string()),
        ("3".to_string(), "Kill la Kill".to_string()),
        ("4".to_string(), "Mob Psycho 100 II".to_string()),
        ("5".to_string(), "My Dress-Up Darling".to_string()),
        ("6".to_string(), "Aku no Hana".to_string()),
        ("7".to_string(), "Eromanga Sensei".to_string()),
        ("8".to_string(), "Yuri on Ice!".to_string()),
    ]
}

const MAX_ENTRIES: usize = 6;

#[component]
pub fn nomination_list(cx: Scope, category: String) -> impl IntoView {
    let (nominees, set_nominees) = create_signal::<Vec<Nominee>>(cx, test_data());
    let (selected_nominee, set_selected_nominee) = create_signal::<Option<Nominee>>(cx, None);

    let insert_nominee = move |value| set_nominees.update(|v| v.push(value));
    let remove_nominee = move |entry_id: &String| {
        set_nominees.update(|v| {
            let index = v.iter().position(|(id, _)| id == entry_id);
            if let Some(index) = index {
                v.remove(index);
            };
        })
    };

    create_effect(cx, move |_| {
        selected_nominee.track();

        if let Some(selected) = selected_nominee.get() {
            insert_nominee(selected);
            set_selected_nominee.set_untracked(None);
        }
    });

    let move_up = move |nominee: MoveArgs| {
        set_nominees.update(|noms| {
            let index = noms.iter().position(|(id, _)| id == &nominee.0);

            if let Some(index) = index {
                if index < 1 {
                    return;
                }

                noms.remove(index);
                noms.insert(index - 1, nominee.clone());
            }
        });
    };

    let move_down = move |nominee: MoveArgs| {
        set_nominees.update(|noms| {
            let index = noms.iter().position(|(id, _)| id == &nominee.0);

            if let Some(index) = index {
                info!("{} {}", index, noms.len());

                if index + 1 >= noms.len() {
                    return;
                }

                noms.remove(index);
                noms.insert(index + 1, nominee.clone())
            }
        })
    };

    let too_many_entries = move || {
        if nominees.get().len() > MAX_ENTRIES {
            format!("Too many entries, should be {}", MAX_ENTRIES)
        } else {
            "".to_string()
        }
    };

    create_effect(cx, move |_| {
        info!("{:#?}", nominees.get());
    });

    view! { cx,
        <h1>{category}</h1>
        <form>
            <ol class="nominees-list">
                <For
                    each=nominees
                    key=|k| k.0.clone()
                    view=move |cx, nom| {
                        let name = nom.1.clone();
                        let nom1 = (nom.0.clone(), nom.1.clone());
                        let nom2 = (nom.0.clone(), nom.1.clone());

                        view! {cx,
                            <Nominee
                                move_up=move |ev| {
                                    ev.prevent_default();
                                    move_up(&nom1);
                                }
                                move_down=move |ev| {
                                    ev.prevent_default();
                                    move_down(&nom2);
                                }
                                remove_item=move |ev| {
                                    ev.prevent_default();
                                    remove_nominee(&nom.0);
                                }
                                name=name
                            />
                        }
                    }
                />
            </ol>

            <output>{too_many_entries}</output>

            <Search insert_nominee=set_selected_nominee />

            <input type="submit" />
        </form>

    }
}
