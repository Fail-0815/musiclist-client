#![feature(async_closure)]

// use gloo_console::log;
use gloo_net::http::Request;
use listclient::{try_login, vote_callback_generator};
use web_sys::HtmlInputElement;
use wishlib::{LoginResult, MusicWish};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct MusicWishListProps {
    songs: UseStateHandle<Vec<MusicWish>>,
}

// TODO: Convert all Async closures into functions and put in lib.rs

#[function_component(LoginFunc)]
fn login_func() -> Html {
    let loggedin = use_state(|| false);
    let loginresult: UseStateHandle<LoginResult> = use_state(|| LoginResult {
        status: false,
        message: "".to_string(),
    });
    {
        let loggedin = loggedin.clone();
        use_effect_with_deps(
            move |_| {
                let loggedin = loggedin.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let loginstate: bool = Request::get("/api/login")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    loggedin.set(loginstate);
                });
                || ()
            },
            (),
        );
    }
    if !*loggedin {
        let input: NodeRef = NodeRef::default();
        let callback = {
            let loginresult = loginresult.clone();
            let input = input.clone();
            Callback::from(move |_| {
                let password = input.cast::<HtmlInputElement>().unwrap().value();
                wasm_bindgen_futures::spawn_local(try_login(loginresult.clone(), password));
            })
        };

        let loginresult = (*loginresult).clone();
        html! {
            <div class="loginpage">
            <h1>{"Hochzeits Musikwunschliste"}</h1>
            <p>{"Passwort kann der Einladung entnommen werden."}</p>
            <div class="input">
            <label>{"Password"}</label>
            <input type = "text" id = "input_password" name = "input_password" ref = {&input}/>
            <p class = "loginresult">{
                if loginresult.status {
                    loggedin.set(true);
                    "Successfull".to_string()
                } else {
                    loginresult.message
                }
            }
            </p>
            <button onclick={callback} >{"Login"}</button>
            </div>
            </div>
        }
    } else {
        html! {
            <MusicListApp />
        }
    }
}

#[function_component(WishList)]
fn wish_list(MusicWishListProps { songs }: &MusicWishListProps) -> Html {
    html! {
        <table>
        <tr>
            <th><p>{"Nr."}</p></th>
            <th><p>{"Titel"}</p></th>
            <th><p>{"Künstler:innen"}</p></th>
            <th><p>{"Kommentare / Sonstiges"}</p></th>
            <th class="score-hdr centered-col"><p>{"Stimmen"}</p></th>
            <th class="centered-col"><p>{"Abstimmen"}</p></th>
        </tr>
        {songs
        .iter()
        .map(|song| {
            html! {
                <tr>
                <th>{song.id}</th>
                <td>{&song.title}</td>
                <td>{&song.artist}</td>
                <td>{&song.comment}</td>
                <td class="score centered-col">{&song.score}</td>
                <td class="centered-col">
                    <button class={if song.voted {"red"} else {""}} onclick={vote_callback_generator(song.id, songs.clone())}>{
                        if song.voted {
                            "-1"
                        } else {
                            "+1"
                        }
                    }
                    </button></td>
                </tr>
            }
        })
        .collect::<Html>()}
        </table>
    }
}

#[function_component(MusicListApp)]
fn app() -> Html {
    let request_url = "/api/wishlist";
    let wishes = use_state(|| vec![]);
    {
        let wishes = wishes.clone();
        use_effect_with_deps(
            move |_| {
                let wishes = wishes.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_wishes: Vec<MusicWish> = Request::get(request_url)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    wishes.set(fetched_wishes);
                });
                || ()
            },
            (),
        );
    }
    let input_artist: NodeRef = NodeRef::default();
    let input_title: NodeRef = NodeRef::default();
    let input_comment: NodeRef = NodeRef::default();
    let onclick = {
        let wishes = wishes.clone();
        let input_artist = input_artist.clone();
        let input_title = input_title.clone();
        let input_comment = input_comment.clone();
        Callback::from(move |_| {
            // Post it to API
            let new_wish = MusicWish {
                id: 99,
                title: input_title.cast::<HtmlInputElement>().unwrap().value(),
                artist: input_artist.cast::<HtmlInputElement>().unwrap().value(),
                comment: input_comment.cast::<HtmlInputElement>().unwrap().value(),
                score: 1,
                voted: true,
            };
            let wishes = wishes.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let posted_wish: MusicWish = Request::post(request_url)
                    .json(&new_wish)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                wishes.set([(*wishes).clone(), vec![posted_wish]].concat());
            });
        })
    };

    html! {
        <>
        <h1>{ "Shitty Music List" }</h1>
        <div class="description" > <p>{ "Lazy solution: Go to spotify and enter your stuff:" }</p>
        <a href="https://open.spotify.com/playlist/5vCJ2gRbjVLopXaCIugx8h?si=4a3e5b6e085b4a11&pt=89b50b0f4b697b6561fde9a97205d1de">{"Link"}</a>
        </div>
        <div>
        <p>{"Neuen Musikwunsch eintragen:"}</p>
        </div>
        <div class="inputmeta">
        <div class="input">
        <label>{"Title"}</label>
        <input type = "text" ref = {input_title}/>
        </div>
        <div class="input">
        <label>{"Artist"}</label>
        <input type = "text" ref = {input_artist}/>
        </div>
        <div class="input">
        <label>{"Comment"}</label>
        <input type = "text" ref = {input_comment}/>
        </div>
        <button {onclick}>{"Abschicken"}</button>
        </div>
        <WishList songs = {wishes} />
        </>
    }
}

fn main() {
    yew::Renderer::<LoginFunc>::new().render();
}
