mod account_html;
pub mod dividers_game;
mod fetch_history;
pub mod game_abstract_class;
mod game_buttons;
mod game_html_template;
pub mod game_utils;
pub mod gcd_game;
mod mainpage_html;
pub mod models;

#[macro_use]
pub mod macros;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};
use web_sys::{MessageEvent, WebSocket};

use crate::dividers_game::GameDividers;
use crate::game_abstract_class::Game;
use crate::gcd_game::GameGcd;

#[wasm_bindgen]
pub fn main(pathname: String) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    create_account_tab(&document, &body);

    if pathname.ends_with("index.html") {
        let game: Rc<dyn Game> = Rc::new(GameGcd);
        initialize_game(&document, &body, &window, &game);
    } else if pathname.ends_with("main.html") {
        mainpage_html::create_main_page(&document, &body);
        setup_websocket();
    } else if pathname.ends_with("dividers") {
        let game: Rc<dyn Game> = Rc::new(GameDividers);
        initialize_game(&document, &body, &window, &game);
    } else if pathname.ends_with("account.html") {
        let storage = window.local_storage().unwrap().unwrap();
        match storage.get_item("token") {
            Ok(Some(_token)) => {
                account_html::create_account_page(&document, &body);
            }
            Ok(None) => {
                account_html::create_login_form(&document, &body);
                // account_html::setup_restart_button(&document, &body, &window);
                account_html::ssetup_login_form(&document, &body, &window);
                account_html::setup_register_button(&document, &body, &window);
            }
            Err(_) => {
                account_html::create_login_form(&document, &body);
                // account_html::setup_restart_button(&document, &body, &window);
                account_html::ssetup_login_form(&document, &body, &window);
            }
        }
        account_html::go_back_to_main_page(&document, &body);
    }

    Ok(())
}

fn initialize_game(
    document: &Document,
    body: &Element,
    window: &web_sys::Window,
    game: &Rc<dyn Game>,
) {
    let secret_value = Rc::new(RefCell::new(42));
    let timer_id = Rc::new(RefCell::new(None));

    let start_game_div = game_html_template::create_startgame_div(document, game).unwrap();
    body.append_child(&start_game_div).unwrap();

    let game_container = game_html_template::create_game_template(document, game).unwrap();
    body.append_child(&game_container).unwrap();

    let answer_box = game_html_template::create_answer_box(document).unwrap();
    body.append_child(&answer_box).unwrap();

    let endgame_div = game_html_template::create_endgame_div(document).unwrap();
    body.append_child(&endgame_div).unwrap();

    game_buttons::setup_start_button(document, &secret_value, &timer_id);
    game_buttons::setup_click_button(document, &secret_value, window, game);
    game_buttons::setup_answer_button(document, &secret_value, &timer_id, window);
    game_buttons::setup_restart_button(document, &secret_value, &timer_id);
    game_buttons::setup_reroll_button(document, &secret_value);
}

fn create_account_tab(document: &Document, body: &Element) {
    let account_link = document.create_element("a").unwrap();
    account_link.set_attribute("id", "account-link").unwrap();
    account_link.set_attribute("href", "/account.html").unwrap();
    account_link.set_inner_html("My Account");
    account_link
        .set_attribute(
            "style",
            "position: absolute; top: 10px; right: 10px; display: block;",
        )
        .unwrap();
    body.append_child(&account_link).unwrap();
}

fn setup_websocket() {
    let ws = WebSocket::new("ws://127.0.0.1:8006/ws/").unwrap();

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Some(txt) = e.data().as_string() {
            web_sys::console::log_1(&format!("New message: {}", txt).into());
            if txt == "Ping" {
                web_sys::window()
                    .unwrap()
                    .alert_with_message("Refresh site. New leaderboard!")
                    .unwrap();
            }
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();
}
