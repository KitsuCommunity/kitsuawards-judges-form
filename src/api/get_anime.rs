use anyhow::Error;
use leptos::{ev::Event, html::Math, leptos_dom::helpers::debounce, *};
use leptos_meta::*;
use leptos_router::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

fn anime_query(title: String) -> String {
    format!(
        r#"{{
            "query": "query {{ \n searchAnimeByTitle(title: \"{}\", first: 20) {{ \n nodes {{ \n id \n slug \n titles \n {{ \n canonical \n }} \n posterImage \n {{ \n views \n {{ \n url \n }} \n }} \n }} \n }} \n }}"
        }}"#,
        title
    )
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AnimeTitleSearchRes {
    pub data: SearchAnimeByTitle,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SearchAnimeByTitle {
    pub searchAnimeByTitle: Nodes,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Nodes {
    pub nodes: Vec<Anime>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Anime {
    pub id: String,
    pub slug: String,
    pub posterImage: PosterImage,
    pub titles: Titles,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Titles {
    pub canonical: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct PosterImage {
    pub views: Vec<Views>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Views {
    pub url: String,
}

fn construct_default_anime(title: String) -> Vec<Anime> {
    let mut v: Vec<Anime> = vec![];

    for t in title.chars() {
        v.push(Anime {
            id: "1".to_string(),
            slug: "https://kitsu.io/".to_string(),
            posterImage: PosterImage {
                views: vec![Views {
                    url: "https://kitsu.io".to_string(),
                }],
            },
            titles: Titles {
                canonical: t.to_string(),
            },
        });
    }

    v
}

pub async fn get_anime(title: String) -> Vec<Anime> {
    log!("fn get_anime: {}", title);

    // return construct_default_anime(title);

    let req = match Request::post("https://kitsu.io/api/graphql")
        .body(anime_query(title.clone()))
        .header("Content-Type", "application/json")
        .send()
        .await
    {
        Ok(val) => val,
        Err(_) => return vec![],
    };

    log!("recieved search: {}", title);

    // let test = &req.text().await?;
    // info!("{}", test);

    match req.json::<AnimeTitleSearchRes>().await {
        Ok(val) => val.data.searchAnimeByTitle.nodes,
        Err(_) => vec![],
    }
}
