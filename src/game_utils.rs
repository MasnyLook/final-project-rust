use wasm_bindgen::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use getrandom::getrandom;
use web_sys::{Document, Element, HtmlInputElement};


pub fn compute_function (input_value : u32, secret_value : u32) -> u32 {
    _gcd(input_value, secret_value)
}

fn _gcd(x : u32, y : u32) -> u32 {
    if x < y {
        return _gcd(y, x);
    }
    if y == 0 {
        return x;
    }
    _gcd(y, x % y)
}

pub fn start_game(document: &Document, secret_value: &Rc<RefCell<u32>>, timer_id: &Rc<RefCell<Option<i32>>>) {
    let start_button = document.get_element_by_id("startButton").unwrap();
    start_button.set_attribute("style", "display: none;").unwrap();

    let game_container = document.get_element_by_id("game").unwrap();
    game_container.set_attribute("style", "display: block;").unwrap();

    let answer_box_container = document.get_element_by_id("answerBoxContainer").unwrap();
    answer_box_container.set_attribute("style", "display: block;").unwrap();

    let answer_box_container = document.get_element_by_id("endgame").unwrap();
    answer_box_container.set_attribute("style", "display: none;").unwrap();

    start_timer(document, timer_id).unwrap();

    let attemps = document.get_element_by_id("attemptsDisplay").unwrap();
    attemps.set_text_content(Some("number of attempts: 0"));

    let timer = document.get_element_by_id("timerDisplay").unwrap();
    timer.set_text_content(Some("Time: 0 seconds"));

    let result = document.get_element_by_id("result").unwrap();
    result.set_text_content(None);

    let input_element = document.get_element_by_id("inputNumber").unwrap();
    input_element.dyn_ref::<HtmlInputElement>().unwrap().set_value("");

    let answerBox = document.get_element_by_id("answerBox").unwrap();
    answerBox.dyn_ref::<HtmlInputElement>().unwrap().set_value("");

    *secret_value.borrow_mut() = generate_random_number() % 100 + 1;
}

pub fn generate_random_number() -> u32 {
    let mut buf = [0u8; 4];
    getrandom(&mut buf).expect("Failed to get random bytes");
    u32::from_le_bytes(buf)
}

fn start_timer(document: &Document, timer_id: &Rc<RefCell<Option<i32>>>) -> Result<(), JsValue> {
    let timer_element = document.get_element_by_id("timerDisplay").unwrap();
    let mut seconds = 0;

    let closure = Closure::wrap(Box::new(move || {
        seconds += 1;
        timer_element.set_text_content(Some(&format!("Time: {} seconds", seconds)));
    }) as Box<dyn FnMut()>);

    let timer = web_sys::window()
        .expect("no global `window` exists")
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000,
        )?;
    closure.forget();

    *timer_id.borrow_mut() = Some(timer);

    Ok(())
}

pub fn update_attempts(document: &Document) {
    let attempts_element = document.get_element_by_id("attemptsDisplay").unwrap();
    let attempts_text = attempts_element.text_content().unwrap();
    let new_attempts = increase_number_of_attempts(&attempts_text);
    attempts_element.set_text_content(Some(&new_attempts));
}

pub fn increase_number_of_attempts(input: &str) -> String {
    // Input: String of the form "number_of_attempts: <number>"
    // Output: String of the form "number_of_attempts: <number+1>"
    let parts: Vec<&str> = input.split(": ").collect();
    if parts.len() != 2 {
        return "Invalid input".to_string();
    }
    let number_of_attempts: u32 = match parts[1].trim().parse() {
        Ok(num) => num,
        Err(_) => return "Invalid number".to_string(),
    };
    format!("number of attempts: {}", number_of_attempts + 1)
}

pub fn check_answer(document: &Document, secret_value: &Rc<RefCell<u32>>, timer_id: &Rc<RefCell<Option<i32>>>) {
    let answer_box = document.get_element_by_id("answerBox").unwrap();
    let answer = answer_box.dyn_ref::<HtmlInputElement>().unwrap().value();
    let answer: u32 = answer.parse().unwrap_or(0);

    let secret_value = *secret_value.borrow();
    if answer == secret_value {
        finish_game(document, timer_id);
    } else {
        let result = document.get_element_by_id("answerResult").unwrap();
        result.set_text_content(Some("Try again!"));
    }
}

fn finish_game(document: &Document, timer_id: &Rc<RefCell<Option<i32>>>) {
    let game_container = document.get_element_by_id("game").unwrap();
    game_container.set_attribute("style", "display: none;").unwrap();

    let answer_box_container = document.get_element_by_id("answerBoxContainer").unwrap();
    answer_box_container.set_attribute("style", "display: none;").unwrap();

    stop_timer(timer_id);

    let end_game_div = document.get_element_by_id("endgame").unwrap();
    end_game_div.set_attribute("style", "display: block;").unwrap();

    let timer_element_text = document.get_element_by_id("timerDisplay").unwrap().text_content().unwrap();
    let total_seconds = get_number_of_seconds(&timer_element_text);
    let total_time_info = document.get_element_by_id("totaltime").unwrap();
    total_time_info.set_text_content(Some(&format!("Total time: {} seconds", total_seconds)));


    let number_of_attempts = document.get_element_by_id("attemptsDisplay").unwrap().text_content().unwrap();
    let total_trials_info = document.get_element_by_id("totaltrials").unwrap();
    total_trials_info.set_text_content(Some(&format!("Total {}", number_of_attempts)));
}

fn get_number_of_seconds(timer_text: &str) -> u32 {
    let parts: Vec<&str> = timer_text.split(" ").collect();
    match parts[1].parse::<u32>() {
        Ok(seconds) => seconds,
        Err(_) => 42,
    }
}

fn stop_timer(timer_id: &Rc<RefCell<Option<i32>>>) {
    web_sys::window()
        .expect("no global `window` exists")
        .clear_interval_with_handle(timer_id.borrow().unwrap());
}
