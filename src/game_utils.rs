use wasm_bindgen::prelude::*;
use chrono::prelude::*;
use getrandom::getrandom;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{Document, HtmlInputElement};
use serde::{Serialize, Deserialize};
use serde_json::json;
use reqwest::Client;
use reqwest::header;
use wasm_bindgen_futures::spawn_local;

pub fn start_game(
    document: &Document,
    secret_value: &Rc<RefCell<u32>>,
    timer_id: &Rc<RefCell<Option<i32>>>,
) {
    let start_button = document.get_element_by_id("startgame").unwrap();
    start_button
        .set_attribute("style", "display: none;")
        .unwrap();

    let game_container = document.get_element_by_id("game").unwrap();
    game_container
        .set_attribute("style", "display: block;")
        .unwrap();

    let answer_box_container = document.get_element_by_id("answerBoxContainer").unwrap();
    answer_box_container
        .set_attribute("style", "display: block;")
        .unwrap();

    let answer_box_container = document.get_element_by_id("endgame").unwrap();
    answer_box_container
        .set_attribute("style", "display: none;")
        .unwrap();

    start_timer(document, timer_id).unwrap();

    let attemps = document.get_element_by_id("attemptsDisplay").unwrap();
    attemps.set_text_content(Some("number of attempts: 0"));

    let timer = document.get_element_by_id("timerDisplay").unwrap();
    timer.set_text_content(Some("Time: 0 seconds"));

    let result = document.get_element_by_id("result").unwrap();
    result.set_text_content(None);

    let input_element = document.get_element_by_id("inputNumber").unwrap();
    input_element
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .set_value("");

    let answer_box = document.get_element_by_id("answerBox").unwrap();
    answer_box
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .set_value("");

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

pub fn check_answer(
    document: &Document,
    secret_value: &Rc<RefCell<u32>>,
    timer_id: &Rc<RefCell<Option<i32>>>,
    window: &web_sys::Window,
) {
    let answer_box = document.get_element_by_id("answerBox").unwrap();
    let answer = answer_box.dyn_ref::<HtmlInputElement>().unwrap().value();
    let answer: u32 = answer.parse().unwrap_or(0);

    let secret_value = *secret_value.borrow();
    if answer == secret_value {
        finish_game(document, timer_id, window);
    } else {
        let result = document.get_element_by_id("answerResult").unwrap();
        result.set_text_content(Some("Try again!"));
    }
}

fn finish_game(document: &Document, timer_id: &Rc<RefCell<Option<i32>>>, window: &web_sys::Window) {
    let game_container = document.get_element_by_id("game").unwrap();
    game_container
        .set_attribute("style", "display: none;")
        .unwrap();

    let answer_box_container = document.get_element_by_id("answerBoxContainer").unwrap();
    answer_box_container
        .set_attribute("style", "display: none;")
        .unwrap();

    stop_timer(timer_id);

    let end_game_div = document.get_element_by_id("endgame").unwrap();
    end_game_div
        .set_attribute("style", "display: block;")
        .unwrap();

    let timer_element_text = document
        .get_element_by_id("timerDisplay")
        .unwrap()
        .text_content()
        .unwrap();
    let total_seconds = get_number_of_seconds(&timer_element_text);
    let total_time_info = document.get_element_by_id("totaltime").unwrap();
    total_time_info.set_text_content(Some(&format!("Total time: {} seconds", total_seconds)));

    let number_of_attempts = document
        .get_element_by_id("attemptsDisplay")
        .unwrap()
        .text_content()
        .unwrap();
    let total_attempts: u32 = get_number_of_attempts(&number_of_attempts);
    let total_trials_info = document.get_element_by_id("totaltrials").unwrap();
    total_trials_info.set_text_content(Some(&format!("Total {}", number_of_attempts)));

    send_game_results_to_server(window, total_seconds, total_attempts, document);
}

#[derive(Deserialize, Debug, Serialize)]
struct AuthenticationToken {
    pub user_name: String,
    pub cookie: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GameResult {
    pub token: AuthenticationToken,
    pub score_time: i32, // can't put u32 in postgresql database
    pub score_moves: i32,
    pub game_type: String,
    pub timestamp: String,
}

fn send_game_results_to_server(window: &web_sys::Window, total_seconds: u32, total_attempts: u32, document: &Document) {
    let storage = window.local_storage().unwrap().unwrap();
    let token = storage.get_item("token").unwrap().unwrap();
    let name = storage.get_item("name").unwrap().unwrap();

    let button = document.get_element_by_id("click").unwrap();
    let button_text = button.text_content().unwrap();
    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    let game_result = GameResult {
        token: AuthenticationToken {
            user_name: name,
            cookie: token,
        },
        score_time: total_seconds as i32,
        score_moves: total_attempts as i32,
        game_type: button_text,
        timestamp: now,
    };

    let request_body = serde_json::to_string(&game_result).unwrap();
    send_message_to_server(window, &request_body);
}

fn send_message_to_server(window: &web_sys::Window, message: &str) {
    let storage = window.local_storage().unwrap().unwrap();
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let window_clone = window.clone();
    let request_body = message.to_string();

    spawn_local(async move {
        let response = client
            .post("http://127.0.0.1:8006/game_result")
            .headers(headers)
            .body(request_body)
            .send()
            .await;            
    });
}

pub fn get_number_of_seconds(timer_text: &str) -> u32 {
    let parts: Vec<&str> = timer_text.split(" ").collect();
    parts[1].parse::<u32>().unwrap_or(42)
}

fn get_number_of_attempts(attempts_info: &str) -> u32 {
    // attempts_info of the form
    // number of attempts: <number>
    let parts: Vec<&str> = attempts_info.split(": ").collect();
    if let Some(number_str) = parts.get(1) {
        if let Ok(number) = number_str.parse::<u32>() {
            return number;
        }
    }
    1000
}

fn stop_timer(timer_id: &Rc<RefCell<Option<i32>>>) {
    web_sys::window()
        .expect("no global `window` exists")
        .clear_interval_with_handle(timer_id.borrow().unwrap());
}
