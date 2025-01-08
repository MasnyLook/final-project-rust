use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

use crate::html;

use crate::game_abstract_class::Game;

pub fn create_startgame_div(document: &Document, game: &Rc<dyn Game>) -> Result<Element, JsValue> {
    let startgame = html!(document, "div", {
        "id" => "startgame",
        "class" => "text-center mt-4"
    });

    let card = html!(document, "div", { // used to manage style on the site
        "class" => "card mb-4 w-50 mx-auto bg-dark text-white"
    });

    let card_body = html!(document, "div", {
        "class" => "card-body"
    });

    let description_text = format!("Welcome to the Guess the Number game! Enter a number x and get the result of {}. If the secret value is too hard to guess, you can reroll it. When you hit the start button, the timer starts.", game.result_string());
    let description = html!(document, "p", {
        "class" => "card-text"
    }, &description_text);
    card_body.append_child(&description)?;

    card.append_child(&card_body)?;
    startgame.append_child(&card)?;

    let start_button = html!(document, "button", {
        "id" => "startButton",
        "class" => "btn btn-primary mt-3"
    }, "Start");
    startgame.append_child(&start_button)?;

    Ok(startgame)
}

pub fn create_endgame_div(document: &Document) -> Result<Element, JsValue> {
    let endgame_container = html!(document, "div", {
        "class" => "d-flex justify-content-center align-items-center vh-100"
    });

    let endgame = html!(document, "div", {
        "id" => "endgame",
        "class" => "container text-center bg-dark text-white p-4"
    });

    let endgame_text = html!(document, "h1", {
        "class" => "display-4 mb-4 text-warning"
    }, "Congratulations! You guessed the number!");
    endgame.append_child(&endgame_text)?;

    let total_time = html!(document, "p", {
        "id" => "totaltime",
        "class" => "lead"
    });
    endgame.append_child(&total_time)?;

    let total_trials = html!(document, "p", {
        "id" => "totaltrials",
        "class" => "lead"
    });
    endgame.append_child(&total_trials)?;

    let restart_button = html!(document, "button", {
        "id" => "restartButton",
        "class" => "btn btn-primary mt-3"
    }, "Restart");
    endgame.append_child(&restart_button)?;

    let link_to_main = html!(document, "a", {
        "href" => "main.html",
        "class" => "d-block mt-3 text-info"
    }, "Go to list of games");
    endgame.append_child(&link_to_main)?;

    endgame_container.append_child(&endgame)?;

    Ok(endgame_container)
}

pub fn create_game_template(document: &Document, game: &Rc<dyn Game>) -> Result<Element, JsValue> {
    let game_container = html!(document, "div", {
        "id" => "game",
        "class" => "container mt-4 mb-4 text-center"
    });

    let val = html!(document, "h1", {
        "class" => "display-4 mb-4 fancy-font"
    }, "Guess the number!");
    game_container.append_child(&val)?;

    let input_group = html!(document, "div", {
        "class" => "input-group mb-3 justify-content-center"
    });

    let input = html!(document, "input", {
        "type" => "number",
        "id" => "inputNumber",
        "placeholder" => "Enter a number",
        "class" => "form-control w50"
    });
    input_group.append_child(&input)?;

    let button = html!(document, "button", {
        "id" => "click",
        "class" => "btn btn-primary ml-2"
    }, game.click_button().as_str());
    input_group.append_child(&button)?;

    game_container.append_child(&input_group)?;

    let result_card = html!(document, "div", {
        "class" => "card bg-dark text-white mt-3 w-50 mx-auto"
    });

    let result_card_body = html!(document, "div", {
        "class" => "card-body"
    });

    let result = html!(document, "p", {
        "id" => "result",
        "class" => "card-text fancy-font"
    });
    result_card_body.append_child(&result)?;

    result_card.append_child(&result_card_body)?;
    game_container.append_child(&result_card)?;

    let attempts_display = html!(document, "div", {
        "id" => "attemptsDisplay",
        "class" => "mt-3 fancy-font"
    }, "number of attempts: 0");
    game_container.append_child(&attempts_display)?;

    let timer_display = html!(document, "div", {
        "id" => "timerDisplay",
        "class" => "mt-3 fancy-font"
    }, "Time: 0 seconds");
    game_container.append_child(&timer_display)?;

    Ok(game_container)
}

pub fn create_answer_box(document: &Document) -> Result<Element, JsValue> {
    let answer_box_container = html!(document, "div", {
        "id" => "answerBoxContainer",
        "class" => "mt-4 text-center fixed-bottom bg-dark text-white"
    });

    let answer_box = html!(document, "input", {
        "type" => "number",
        "id" => "answerBox",
        "placeholder" => "Enter an answer",
        "class" => "form-control w-25 mx-auto mb-3"
    });
    answer_box_container.append_child(&answer_box)?;

    let submit_button = html!(document, "button", {
        "id" => "submitButton",
        "class" => "btn btn-primary mx-2"
    }, "Submit answer");
    answer_box_container.append_child(&submit_button)?;

    let answer_result = html!(document, "p", {
        "id" => "answerResult",
        "class" => "mt-3"
    });
    answer_box_container.append_child(&answer_result)?;

    let reroll_button = html!(document, "button", {
        "id" => "rerollButton",
        "class" => "btn btn-secondary mx-2"
    }, "Reroll secret value");
    answer_box_container.append_child(&reroll_button)?;

    Ok(answer_box_container)
}
