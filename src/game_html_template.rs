use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

use crate::game_abstract_class::Game;

pub fn create_startgame_div(document: &Document, game: &Rc<dyn Game>) -> Result<Element, JsValue> {
    let startgame = document.create_element("div")?;
    startgame.set_attribute("id", "startgame")?;
    startgame.set_attribute("class", "text-center mt-4")?;

    let card = document.create_element("div")?;
    card.set_attribute("class", "card mb-4 w-50 mx-auto bg-dark text-white")?;

    let card_body = document.create_element("div")?;
    card_body.set_attribute("class", "card-body")?;

    let description = document.create_element("p")?;
    let description_text = format!("Welcome to the Guess the Number game! Enter a number x and get the result of {}. If the secret value is too hard to guess, you can reroll it. When you hit the start button, the timer starts.", game.result_string());
    description.set_text_content(Some(&description_text));
    description.set_attribute("class", "card-text")?;
    card_body.append_child(&description)?;

    card.append_child(&card_body)?;
    startgame.append_child(&card)?;

    let start_button = document.create_element("button")?;
    start_button.set_attribute("id", "startButton")?;
    start_button.set_text_content(Some("Start"));
    startgame.append_child(&start_button)?;
    start_button.set_attribute("class", "btn btn-primary mt-3")?;

    Ok(startgame)
}

pub fn create_endgame_div(document: &Document) -> Result<Element, JsValue> {
    let endgame_container = document.create_element("div")?;
    endgame_container.set_attribute(
        "class",
        "d-flex justify-content-center align-items-center vh-100",
    )?;

    let endgame = document.create_element("div")?;
    endgame.set_attribute("id", "endgame")?;
    endgame.set_attribute("class", "container text-center bg-dark text-white p-4")?;

    let endgame_text = document.create_element("h1")?;
    endgame_text.set_text_content(Some("Congratulations! You guessed the number!"));
    endgame_text.set_attribute("class", "display-4 mb-4 text-warning")?;
    endgame.append_child(&endgame_text)?;

    let total_time = document.create_element("p")?;
    total_time.set_attribute("id", "totaltime")?;
    total_time.set_attribute("class", "lead")?;
    endgame.append_child(&total_time)?;

    let total_trials = document.create_element("p")?;
    total_trials.set_attribute("id", "totaltrials")?;
    total_trials.set_attribute("class", "lead")?;
    endgame.append_child(&total_trials)?;

    let restart_button = document.create_element("button")?;
    restart_button.set_attribute("id", "restartButton")?;
    restart_button.set_text_content(Some("Restart"));
    restart_button.set_attribute("class", "btn btn-primary mt-3")?;
    endgame.append_child(&restart_button)?;

    let link_to_main = document.create_element("a")?;
    link_to_main.set_attribute("href", "main.html")?;
    link_to_main.set_text_content(Some("Go to list of games"));
    link_to_main.set_attribute("class", "d-block mt-3 text-info")?;
    endgame.append_child(&link_to_main)?;

    endgame_container.append_child(&endgame)?;

    Ok(endgame_container)
}

pub fn create_game_template(document: &Document, game: &Rc<dyn Game>) -> Result<Element, JsValue> {
    let game_container = document.create_element("div")?;
    game_container.set_attribute("id", "game")?;
    game_container.set_attribute("class", "container mt-4 mb-4 text-center")?;

    let val = document.create_element("h1")?;
    val.set_text_content(Some("Guess the number!"));
    val.set_attribute("class", "display-4 mb-4 fancy-font")?;
    game_container.append_child(&val)?;

    let input_group = document.create_element("div")?;
    input_group.set_attribute("class", "input-group mb-3 justify-content-center")?;

    let input = document.create_element("input")?;
    input.set_attribute("type", "number")?;
    input.set_attribute("id", "inputNumber")?;
    input.set_attribute("placeholder", "Enter a number")?;
    input.set_attribute("class", "form-control w50")?;
    input_group.append_child(&input)?;

    let button = document.create_element("button")?;
    button.set_attribute("id", "click")?;
    button.set_text_content(Some(game.click_button().as_str()));
    button.set_attribute("class", "btn btn-primary ml-2")?;
    input_group.append_child(&button)?;

    game_container.append_child(&input_group)?;

    let result_card = document.create_element("div")?;
    result_card.set_attribute("class", "card bg-dark text-white mt-3 w-50 mx-auto")?;

    let result_card_body = document.create_element("div")?;
    result_card_body.set_attribute("class", "card-body")?;

    let result = document.create_element("p")?;
    result.set_attribute("id", "result")?;
    result.set_attribute("class", "card-text fancy-font")?;
    result_card_body.append_child(&result)?;

    result_card.append_child(&result_card_body)?;
    game_container.append_child(&result_card)?;

    let attempts_display = document.create_element("div")?;
    attempts_display.set_attribute("id", "attemptsDisplay")?;
    attempts_display.set_text_content(Some("number of attempts: 0"));
    attempts_display.set_attribute("class", "mt-3 fancy-font")?;
    game_container.append_child(&attempts_display)?;

    let timer_display = document.create_element("div")?;
    timer_display.set_attribute("id", "timerDisplay")?;
    timer_display.set_text_content(Some("Time: 0 seconds"));
    timer_display.set_attribute("class", "mt-3 fancy-font")?;
    game_container.append_child(&timer_display)?;

    Ok(game_container)
}

pub fn create_answer_box(document: &Document) -> Result<Element, JsValue> {
    let answer_box_container = document.create_element("div")?;
    answer_box_container.set_attribute("id", "answerBoxContainer")?;
    answer_box_container
        .set_attribute("class", "mt-4 text-center fixed-bottom bg-dark text-white")?;

    let answer_box = document.create_element("input")?;
    answer_box.set_attribute("type", "number")?;
    answer_box.set_attribute("id", "answerBox")?;
    answer_box.set_attribute("placeholder", "Enter an answer")?;
    answer_box.set_attribute("class", "form-control w-25 mx-auto mb-3")?;
    answer_box_container.append_child(&answer_box)?;

    let submit_button = document.create_element("button")?;
    submit_button.set_attribute("id", "submitButton")?;
    submit_button.set_text_content(Some("Submit answer"));
    submit_button.set_attribute("class", "btn btn-primary mx-2")?;
    answer_box_container.append_child(&submit_button)?;

    let answer_result = document.create_element("p")?;
    answer_result.set_attribute("id", "answerResult")?;
    answer_result.set_attribute("class", "mt-3")?;
    answer_box_container.append_child(&answer_result)?;

    let reroll_button = document.create_element("button")?;
    reroll_button.set_attribute("id", "rerollButton")?;
    reroll_button.set_text_content(Some("Reroll secret value"));
    reroll_button.set_attribute("class", "btn btn-secondary mx-2")?;
    answer_box_container.append_child(&reroll_button)?;

    Ok(answer_box_container)
}
