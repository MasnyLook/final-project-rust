use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::*;

extern crate websys;
use websys::game_utils;

// TODO:
// Fix webdriver.json file or describe a way to configure the webdriver
// https://rustwasm.github.io/docs/book/game-of-life/testing.html

wasm_bindgen_test_configure!(run_in_browser);

#[allow(dead_code)]
#[wasm_bindgen_test]
fn test_start_game() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let secret_value = Rc::new(RefCell::new(42));
    let timer_id = Rc::new(RefCell::new(None));

    // Create necessary elements
    let start_button = document.create_element("button").unwrap();
    start_button.set_attribute("id", "startButton").unwrap();
    document
        .body()
        .unwrap()
        .append_child(&start_button)
        .unwrap();

    let game_container = document.create_element("div").unwrap();
    game_container.set_attribute("id", "game").unwrap();
    document
        .body()
        .unwrap()
        .append_child(&game_container)
        .unwrap();

    let answer_box_container = document.create_element("div").unwrap();
    answer_box_container
        .set_attribute("id", "answerBoxContainer")
        .unwrap();
    document
        .body()
        .unwrap()
        .append_child(&answer_box_container)
        .unwrap();

    let timer_display = document.create_element("div").unwrap();
    timer_display.set_attribute("id", "timerDisplay").unwrap();
    document
        .body()
        .unwrap()
        .append_child(&timer_display)
        .unwrap();

    game_utils::start_game(&document, &secret_value, &timer_id);

    assert_eq!(
        start_button.get_attribute("style").unwrap(),
        "display: none;"
    );
    assert_eq!(
        game_container.get_attribute("style").unwrap(),
        "display: block;"
    );
    assert_eq!(
        answer_box_container.get_attribute("style").unwrap(),
        "display: block;"
    );
}
