mod game;
mod game_html_template;

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

    // let start_button = document.create_element("button")?;
    // start_button.set_attribute("id", "startButton")?;
    // start_button.set_text_content(Some("Start"));
    // body.append_child(&start_button)?;

    let start_game_div = game_html_template::create_startgame_div(&document)?;
    body.append_child(&start_game_div)?;

    let game_container = document.create_element("div")?;
    game_container.set_attribute("id", "game")?;

    // Manufacture the element we're gonna append
    let val = document.create_element("h1")?;
    val.set_text_content(Some("Guess the number!"));
    game_container.append_child(&val)?;

     // Create and append the input element
     let input = document.create_element("input")?;
     input.set_attribute("type", "number")?;
     input.set_attribute("id", "inputNumber")?;
     input.set_attribute("placeholder", "Enter a number")?;
     game_container.append_child(&input)?;
 
     // Create and append the button element
     let button = document.create_element("button")?;
     button.set_attribute("id", "click")?;
     button.set_text_content(Some("gcd"));
     game_container.append_child(&button)?;
 
     // Create and append the result paragraph
     let result = document.create_element("p")?;
     result.set_attribute("id", "result")?;
     game_container.append_child(&result)?;
 
     // Create and append the attempts display div
     let attempts_display = document.create_element("div")?;
     attempts_display.set_attribute("id", "attemptsDisplay")?;
     attempts_display.set_text_content(Some("number of attempts: 0"));
     game_container.append_child(&attempts_display)?;

    let secret_value_clone = Rc::clone(&secret_value);
    let timer_id_clone = Rc::clone(&timer_id);
    let document_clone = document.clone();
    let start_closure = Closure::wrap(Box::new(move || {
        start_game(&document_clone, &secret_value_clone, &timer_id_clone);
    }) as Box<dyn FnMut()>);

    let start_button = document.get_element_by_id("startButton").unwrap();
    start_button.add_event_listener_with_callback("click", start_closure.as_ref().unchecked_ref())?;
    start_closure.forget();

     let document_clone = document.clone();
     let secret_value_clone = Rc::clone(&secret_value);
     let closure = Closure::wrap(Box::new(move || {
         let input_element = document_clone.get_element_by_id("inputNumber").unwrap();
         let input_value = input_element.dyn_ref::<HtmlInputElement>().unwrap().value();
         let input_number: u32 = input_value.parse().unwrap_or(0);
 
         let result = game::compute_function(input_number, *secret_value_clone.borrow());
         let result_element = document_clone.get_element_by_id("result").unwrap();
         result_element.set_text_content(Some(&format!("Result: {}", result)));

        update_attempts(&document_clone);
     }) as Box<dyn FnMut()>);
 
     button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
     closure.forget();

     let timer_display = document.create_element("div")?;
     timer_display.set_attribute("id", "timerDisplay")?;
     timer_display.set_text_content(Some("Time: 0 seconds"));
     game_container.append_child(&timer_display)?;

    body.append_child(&game_container)?;

    let answer_box = create_answer_box(&document)?;
    body.append_child(&answer_box)?;

    let closure = check_answer_closure(&document, &secret_value, &timer_id);
    let submit_button = document.get_element_by_id("submitButton").unwrap();
    submit_button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn create_answer_box(document: &Document) -> Result<Element, JsValue> {
    let answer_box_container = document.create_element("div")?;
    answer_box_container.set_attribute("id", "answerBoxContainer")?;

    let answer_box = document.create_element("input")?;
    answer_box.set_attribute("type", "number")?;
    answer_box.set_attribute("id", "answerBox")?;
    answer_box.set_attribute("placeholder", "Enter a number")?;
    answer_box_container.append_child(&answer_box)?;

    let submit_button = document.create_element("button")?;
    submit_button.set_attribute("id", "submitButton")?;
    submit_button.set_text_content(Some("Submit"));
    answer_box_container.append_child(&submit_button)?;

    let answer_result = document.create_element("p")?;
    answer_result.set_attribute("id", "answerResult")?;
    answer_box_container.append_child(&answer_result)?;

    Ok(answer_box_container)
}

fn check_answer_closure(document: &Document, secret_value: &Rc<RefCell<u32>>, timer_id: &Rc<RefCell<Option<i32>>>) -> Closure<dyn FnMut()> {
    let document_clone = document.clone();
    let secret_value_clone = Rc::clone(&secret_value);
    let timer_id_clone = Rc::clone(&timer_id);
    Closure::wrap(Box::new(move || {
        check_answer(&document_clone, &secret_value_clone, &timer_id_clone);
    }) as Box<dyn FnMut()>)
}

fn check_answer(document: &Document, secret_value: &Rc<RefCell<u32>>, timer_id: &Rc<RefCell<Option<i32>>>) {
    let answer_box = document.get_element_by_id("answerBox").unwrap();
    let answer = answer_box.dyn_ref::<HtmlInputElement>().unwrap().value();
    let answer: u32 = answer.parse().unwrap_or(0);

    let secret_value = *secret_value.borrow();
    if answer == secret_value {
        // let game_container = document.get_element_by_id("game").unwrap();
        // game_container.set_attribute("style", "display: none;").unwrap();


        // let answer_box_container = document.get_element_by_id("answerBoxContainer").unwrap();
        // answer_box_container.set_attribute("style", "display: none;").unwrap();

        let result = document.get_element_by_id("answerResult").unwrap();
        result.set_text_content(Some("Congratulations! You guessed the number!"));
        stop_timer(timer_id);
    } else {
        let result = document.get_element_by_id("answerResult").unwrap();
        result.set_text_content(Some("Try again!"));
    }
}

fn start_game(document: &Document, secret_value: &Rc<RefCell<u32>>, timer_id: &Rc<RefCell<Option<i32>>>) {
    let start_button = document.get_element_by_id("startButton").unwrap();
    start_button.set_attribute("style", "display: none;").unwrap();

    let game_container = document.get_element_by_id("game").unwrap();
    game_container.set_attribute("style", "display: block;").unwrap();

    let answer_box_container = document.get_element_by_id("answerBoxContainer").unwrap();
    answer_box_container.set_attribute("style", "display: block;").unwrap();

    start_timer(document, timer_id).unwrap();

    *secret_value.borrow_mut() = generate_random_number() % 100 + 1;
}

fn update_attempts(document: &Document) {
    let attempts_element = document.get_element_by_id("attemptsDisplay").unwrap();
    let attempts_text = attempts_element.text_content().unwrap();
    let new_attempts = increase_number_of_attempts(&attempts_text);
    attempts_element.set_text_content(Some(&new_attempts));
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

fn stop_timer(timer_id: &Rc<RefCell<Option<i32>>>) {
    web_sys::window()
        .expect("no global `window` exists")
        .clear_interval_with_handle(timer_id.borrow().unwrap());
}

fn append_paragraph(document: &Document, parent: &Element, text: &str) -> Result<(), JsValue> {
    let val = document.create_element("p")?;
    val.set_text_content(Some(text));
    val.set_attribute("id", "hello")?;
    parent.append_child(&val)?;
    Ok(())
}

#[wasm_bindgen]
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

pub fn generate_random_number() -> u32 {
    let mut buf = [0u8; 4];
    getrandom(&mut buf).expect("Failed to get random bytes");
    u32::from_le_bytes(buf)
}