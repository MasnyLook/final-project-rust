use crate::models;
use chrono::prelude::*;
use getrandom::getrandom;
use reqwest::header;
use reqwest::Client;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, HtmlInputElement};

use crate::set_element_style;
use crate::set_element_text;

pub fn start_game(
    document: &Document,
    secret_value: &Rc<RefCell<u32>>,
    timer_id: &Rc<RefCell<Option<i32>>>,
) {
    set_element_style!(document, "startgame", "display: none;");
    set_element_style!(document, "game", "display: block;");
    set_element_style!(document, "answerBoxContainer", "display: block;");
    set_element_style!(document, "endgame", "display: none;");

    start_timer(document, timer_id).unwrap();

    set_element_text!(document, "attemptsDisplay", "number of attempts: 0");
    set_element_text!(document, "timerDisplay", "Time: 0 seconds");
    set_element_text!(document, "result", "");

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
        set_element_text!(document, "answerResult", "Try again!");
        update_attempts(document);
    }
}

fn finish_game(document: &Document, timer_id: &Rc<RefCell<Option<i32>>>, window: &web_sys::Window) {
    set_element_style!(document, "game", "display: none;");
    set_element_style!(document, "answerBoxContainer", "display: none;");
    set_element_style!(document, "endgame", "display: block;");

    stop_timer(timer_id);

    let timer_element_text = document
        .get_element_by_id("timerDisplay")
        .unwrap()
        .text_content()
        .unwrap();
    let total_seconds = get_number_of_seconds(&timer_element_text);
    set_element_text!(
        document,
        "totaltime",
        &format!("Total time: {} seconds", total_seconds)
    );

    let number_of_attempts = document
        .get_element_by_id("attemptsDisplay")
        .unwrap()
        .text_content()
        .unwrap();
    let total_attempts: u32 = get_number_of_attempts(&number_of_attempts);
    set_element_text!(
        document,
        "totaltrials",
        &format!("Total {}", number_of_attempts)
    );

    send_game_results_to_server(window, total_seconds, total_attempts, document);
}

fn send_game_results_to_server(
    window: &web_sys::Window,
    total_seconds: u32,
    total_attempts: u32,
    document: &Document,
) {
    let storage = window.local_storage().unwrap().unwrap();
    let token = match storage.get_item("token") {
        Ok(Some(token)) => token,
        Ok(None) | Err(_) => {
            web_sys::console::log_1(
                &"We don't send result to the server, as the user is logged out.".into(),
            );
            return;
        }
    };
    let name = storage.get_item("name").unwrap().unwrap(); // if token exists, then name also

    let button = document.get_element_by_id("click").unwrap();
    let button_text = button.text_content().unwrap();
    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    let game_result = models::GameResultBody {
        token: models::AuthenticationToken {
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

fn send_message_to_server(_window: &web_sys::Window, message: &str) {
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let request_body = message.to_string();

    spawn_local(async move {
        if let Err(err) = client
            .post("http://127.0.0.1:8006/game_result")
            .headers(headers)
            .body(request_body)
            .send()
            .await
        {
            web_sys::console::log_1(&format!("Failed to send game result: {:?}", err).into());
        }
    });
}

pub fn get_number_of_seconds(timer_text: &str) -> u32 {
    // timer_text of the form: "Time: <number> seconds"
    let parts: Vec<&str> = timer_text.split(" ").collect();
    parts[1].parse::<u32>().unwrap_or(1000)
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
