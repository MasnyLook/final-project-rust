use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlInputElement};

pub fn create_main_page(document: &Document, body: &Element) {
    let main_page = document.create_element("div").unwrap();
    main_page.set_attribute("id", "mainpage").unwrap();
    main_page.set_attribute("style", "text-align: center; margin-top: 20px;").unwrap();

    let description = document.create_element("p").unwrap();
    description.set_inner_html(
        "<strong>Welcome to the <em>Guess the Number</em> game!</strong> <br>
        Enter a number and get the result of <strong>gcd(x, secret_value)</strong>. <br>
        If the secret value is too hard to guess, you can <em>reroll</em> it. <br>
        When you hit the <strong>start</strong> button, the timer starts."
    );
    description.set_attribute("style", "margin-bottom: 20px; font-size: 18px; color: #333; line-height: 1.5;").unwrap();
    main_page.append_child(&description).unwrap();
    
    let games_list_description = document.create_element("p").unwrap();
    games_list_description.set_text_content(Some("This is the site with a list of games."));
    games_list_description.set_attribute("style", "margin-bottom: 20px; font-size: 16px; color: #555;").unwrap();
    main_page.append_child(&games_list_description).unwrap();

    let games_list = document.create_element("ul").unwrap();
    games_list.set_attribute("style", "list-style-type: none; padding: 0;").unwrap();

    let games = vec![
        ("gcd", "index.html"),
        ("dividers", "dividers"),
    ];

    for (name, url) in games {
        let list_item = document.create_element("li").unwrap();
        list_item.set_attribute("style", "margin-bottom: 10px;").unwrap();

        let link = document.create_element("a").unwrap();
        link.set_attribute("href", url).unwrap();
        link.set_text_content(Some(name));
        link.set_attribute("style", "font-size: 16px; color: #1E90FF; text-decoration: none;").unwrap();

        list_item.append_child(&link).unwrap();
        games_list.append_child(&list_item).unwrap();
    }

    main_page.append_child(&games_list).unwrap();
    
    body.append_child(&main_page).unwrap();
}