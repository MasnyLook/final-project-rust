use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlInputElement};

pub fn create_startgame_div(document: &Document) -> Result<Element, JsValue> {
    let startgame = document.create_element("div")?;
    startgame.set_attribute("id", "startgame")?;
    
    let start_button = document.create_element("button")?;
    start_button.set_attribute("id", "startButton")?;
    start_button.set_text_content(Some("Start"));
    startgame.append_child(&start_button)?;

    Ok(startgame)
}

pub fn create_endgame_div(document: &Document) -> Result<Element, JsValue> {
    let endgame = document.create_element("div")?;
    endgame.set_attribute("id", "endgame")?;
    
    let endgame_text = document.create_element("h1")?;
    endgame_text.set_text_content(Some("Congratulations! You guessed the number!"));
    endgame.append_child(&endgame_text)?;

    Ok(endgame)
}
