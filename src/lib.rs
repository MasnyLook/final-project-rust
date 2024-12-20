pub mod dividers_game;
pub mod game_abstract_class;
mod game_buttons;
mod game_html_template;
pub mod game_utils;
pub mod gcd_game;
mod mainpage_html;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

use crate::dividers_game::GameDividers;
use crate::game_abstract_class::Game;
use crate::gcd_game::GameGcd;

#[wasm_bindgen]
pub fn main(pathname: String) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    if pathname.ends_with("index.html") {
        let game: Rc<dyn Game> = Rc::new(GameGcd);
        initialize_game(&document, &body, &window, &game);
    } else if pathname.ends_with("main.html") {
        mainpage_html::create_main_page(&document, &body);
    } else if pathname.ends_with("dividers") {
        let game: Rc<dyn Game> = Rc::new(GameDividers);
        initialize_game(&document, &body, &window, &game);
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
    game_buttons::setup_answer_button(document, &secret_value, &timer_id);
    game_buttons::setup_restart_button(document, &secret_value, &timer_id);
    game_buttons::setup_reroll_button(document, &secret_value);
}
