use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlInputElement};

pub fn create_startgame_div(document: &Document) -> Result<Element, JsValue> {
    let startgame = document.create_element("div")?;
    startgame.set_attribute("id", "startgame")?;
    startgame.set_attribute("style", "text-align: center; margin-top: 20px;")?;

    let description = document.create_element("p")?;
    description.set_text_content(Some("Welcome to the Guess the Number game! Enter a number and get the result of gcd(x, secret_value). If the secret value is too hard to guess, you can reroll it. When you hit the start button, the timer starts."));
    description.set_attribute("style", "margin-bottom: 20px; font-size: 16px;")?;
    startgame.append_child(&description)?;
    
    let start_button = document.create_element("button")?;
    start_button.set_attribute("id", "startButton")?;
    start_button.set_text_content(Some("Start"));
    startgame.append_child(&start_button)?;
    start_button.set_attribute("style", "padding: 10px 20px; font-size: 16px; cursor: pointer;")?;

    Ok(startgame)
}

pub fn create_endgame_div(document: &Document) -> Result<Element, JsValue> {
    let endgame = document.create_element("div")?;
    endgame.set_attribute("id", "endgame")?;
    
    let endgame_text = document.create_element("h1")?;
    endgame_text.set_text_content(Some("Congratulations! You guessed the number!"));
    endgame.append_child(&endgame_text)?;

    let total_time = document.create_element("p")?;
    total_time.set_attribute("id", "totaltime")?;
    endgame.append_child(&total_time)?;

    let total_trials = document.create_element("p")?;
    total_trials.set_attribute("id", "totaltrials")?;
    endgame.append_child(&total_trials)?;

    let restart_button = document.create_element("button")?;
    restart_button.set_attribute("id", "restartButton")?;
    restart_button.set_text_content(Some("Restart"));
    endgame.append_child(&restart_button)?;

    Ok(endgame)
}

pub fn create_game_template(document: &Document) -> Result<Element, JsValue> {
    let game_container = document.create_element("div")?;
    game_container.set_attribute("id", "game")?;
    
    let val = document.create_element("h1")?;
    val.set_text_content(Some("Guess the number!"));
    game_container.append_child(&val)?;

    let input = document.create_element("input")?;
    input.set_attribute("type", "number")?;
    input.set_attribute("id", "inputNumber")?;
    input.set_attribute("placeholder", "Enter a number")?;
    game_container.append_child(&input)?;

    let button = document.create_element("button")?;
    button.set_attribute("id", "click")?;
    button.set_text_content(Some("gcd"));
    game_container.append_child(&button)?;

    let result = document.create_element("p")?;
    result.set_attribute("id", "result")?;
    game_container.append_child(&result)?;

    let attempts_display = document.create_element("div")?;
    attempts_display.set_attribute("id", "attemptsDisplay")?;
    attempts_display.set_text_content(Some("number of attempts: 0"));
    game_container.append_child(&attempts_display)?;

    let timer_display = document.create_element("div")?;
    timer_display.set_attribute("id", "timerDisplay")?;
    timer_display.set_text_content(Some("Time: 0 seconds"));
    game_container.append_child(&timer_display)?;

    Ok(game_container)
}

pub fn create_answer_box(document: &Document) -> Result<Element, JsValue> {
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

    let reroll_button = document.create_element("button")?;
    reroll_button.set_attribute("id", "rerollButton")?;
    reroll_button.set_text_content(Some("Reroll"));
    answer_box_container.append_child(&reroll_button)?;

    Ok(answer_box_container)
}
