mod game_buttons;
mod game_html_template;
pub mod game_utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use getrandom::getrandom;
use web_sys::{Document, Element, HtmlInputElement};

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let secret_value = Rc::new(RefCell::new(42));
    let timer_id = Rc::new(RefCell::new(None));

    let start_game_div = game_html_template::create_startgame_div(&document)?;
    body.append_child(&start_game_div)?;

    let game_container = game_html_template::create_game_template(&document)?;
    body.append_child(&game_container)?;

    let answer_box = game_html_template::create_answer_box(&document)?;
    body.append_child(&answer_box)?;

    let endgame_div = game_html_template::create_endgame_div(&document)?;
    body.append_child(&endgame_div)?;

    game_buttons::setup_start_button(&document, &secret_value, &timer_id);
    game_buttons::setup_click_button(&document, &secret_value);
    game_buttons::setup_answer_button(&document, &secret_value, &timer_id);
    game_buttons::setup_restart_button(&document, &secret_value, &timer_id);
    game_buttons::setup_reroll_button(&document, &secret_value);

    Ok(())
}

pub fn clear_input_field(document: &Document, input_id: &str) {
    if let Some(input_element) = document.get_element_by_id(input_id) {
        if let Some(input) = input_element.dyn_ref::<HtmlInputElement>() {
            input.set_value("");
        }
    }
}
