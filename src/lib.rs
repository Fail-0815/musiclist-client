use gloo_net::http::Request;
use wishlib::{LoginResult, MusicWish};
use yew::prelude::*;

pub async fn try_login(state: UseStateHandle<LoginResult>, password: String) {
    let resp = Request::post("/api/login")
        .json(&password)
        .unwrap()
        .send()
        .await
        .unwrap();
    state.set(resp.json().await.unwrap());
}

async fn vote_wish(id: i32, songs: UseStateHandle<Vec<MusicWish>>) {
    let url = format!("/api/wishlist/{}/vote", id);
    let resp: Option<bool> = Request::post(url.as_str())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let votestate = resp.expect("Service returned null instead of result. Maybe lost Session ID?");
    let op = |x: usize| if votestate { x + 1 } else { x - 1 };
    // TODO: Should be a Map instead of Vec.
    let newsongs = songs
        .iter()
        .map(|wish| {
            if wish.id == id {
                MusicWish {
                    id: wish.id,
                    title: wish.title.clone(),
                    artist: wish.artist.clone(),
                    comment: wish.comment.clone(),
                    voted: votestate,
                    score: op(wish.score),
                }
            } else {
                wish.clone()
            }
        })
        .collect();
    songs.set(newsongs);
}

pub fn vote_callback_generator(
    id: i32,
    songs: UseStateHandle<Vec<MusicWish>>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let songs = songs.clone();
        wasm_bindgen_futures::spawn_local(vote_wish(id, songs));
    })
}
