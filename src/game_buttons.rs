use crate::game_utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlInputElement};


pub fn setup_start_button(document: &Document, secret_value: &Rc<RefCell<u32>>, timer_id: &Rc<RefCell<Option<i32>>>) {
    let secret_value_clone = Rc::clone(&secret_value);
    let timer_id_clone = Rc::clone(&timer_id);
    let document_clone = document.clone();
    let start_closure = Closure::wrap(Box::new(move || {
        game_utils::start_game(&document_clone, &secret_value_clone, &timer_id_clone);
    }) as Box<dyn FnMut()>);

    let start_button = document.get_element_by_id("startButton").expect("startButton element not found");
    start_button.add_event_listener_with_callback("click", start_closure.as_ref().unchecked_ref()).expect("Failed to add event listener");
    start_closure.forget();
}

pub fn setup_click_button(document: &Document, secret_value: &Rc<RefCell<u32>>) {
    let document_clone = document.clone();
    let secret_value_clone = Rc::clone(&secret_value);
    let closure = Closure::wrap(Box::new(move || {
        let input_element = document_clone.get_element_by_id("inputNumber").expect("inputNumber element not found");
        let input_value = input_element.dyn_ref::<HtmlInputElement>().expect("inputNumber is not an HtmlInputElement").value();
        let input_number: u32 = input_value.parse().unwrap_or(0);

        let result = game_utils::compute_function(input_number, *secret_value_clone.borrow());
        let result_element = document_clone.get_element_by_id("result").expect("result element not found");
        result_element.set_text_content(Some(&format!("Result: {}", result)));

        game_utils::update_attempts(&document_clone);
    }) as Box<dyn FnMut()>);

    let button = document.get_element_by_id("click").expect("click button not found");
    button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).expect("Failed to add event listener");
    closure.forget();
}

pub fn setup_answer_button(document: &Document, secret_value: &Rc<RefCell<u32>>, timer_id: &Rc<RefCell<Option<i32>>>) {
    let document_clone = document.clone();
    let secret_value_clone = Rc::clone(&secret_value);
    let timer_id_clone = Rc::clone(&timer_id);
    let closure = Closure::wrap(Box::new(move || {
        game_utils::check_answer(&document_clone, &secret_value_clone, &timer_id_clone);
    }) as Box<dyn FnMut()>);

    let submit_button = document.get_element_by_id("submitButton").unwrap();
    submit_button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn setup_restart_button(document: &Document, secret_value: &Rc<RefCell<u32>>, timer_id: &Rc<RefCell<Option<i32>>>) {
    let secret_value_clone = Rc::clone(&secret_value);
    let timer_id_clone = Rc::clone(&timer_id);
    let document_clone = document.clone();
    let start_closure = Closure::wrap(Box::new(move || {
        game_utils::start_game(&document_clone, &secret_value_clone, &timer_id_clone);
    }) as Box<dyn FnMut()>);

    // check if yu can set given attribute twice
    let start_button = document.get_element_by_id("restartButton").expect("startButton element not found");
    start_button.add_event_listener_with_callback("click", start_closure.as_ref().unchecked_ref()).expect("Failed to add event listener");
    start_closure.forget();
}

pub fn setup_reroll_button(document: &Document, secret_value: &Rc<RefCell<u32>>) {
    let secret_value_clone = Rc::clone(&secret_value);
    let document_clone = document.clone();
    let reroll_closure = Closure::wrap(Box::new(move || {
        *secret_value_clone.borrow_mut() = game_utils::generate_random_number() % 100 + 1;
        let result = document_clone.get_element_by_id("result").expect("result element not found");
        result.set_text_content(Some("Rerolled!"));
        game_utils::update_attempts(&document_clone);
    }) as Box<dyn FnMut()>);

    let reroll_button = document.get_element_by_id("rerollButton").expect("rerollButton element not found");
    reroll_button.add_event_listener_with_callback("click", reroll_closure.as_ref().unchecked_ref()).expect("Failed to add event listener");
    reroll_closure.forget();
}
